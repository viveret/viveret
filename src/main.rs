use std::{
    cell::RefCell, collections::HashMap, fs::{self, File}, 
    path::{Path, PathBuf}, rc::Rc, time::SystemTime
};

use chrono::{DateTime, Local};
use pulldown_cmark::{html, Options, Parser};
use serde_yaml::Value;

// ========== Data Structures ==========

type FrontMatter = HashMap<String, String>;

#[derive(Debug)]
struct TemplateContext {
    strings: HashMap<String, String>,
    nodes: HashMap<String, Rc<TemplateNode>>,
    json_data: HashMap<String, Value>,
    parent: Option<Rc<RefCell<TemplateContext>>>,
}

impl TemplateContext {
    pub fn new(parent: Option<Rc<RefCell<TemplateContext>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            strings: HashMap::new(),
            nodes: HashMap::new(),
            json_data: HashMap::new(),
            parent,
        }))
    }
    
    pub fn add_front_matter(&mut self, front_matter: &FrontMatter) {
        for (k, v) in front_matter.iter() {
            self.strings.insert(k.clone(), v.clone());
        }
    }
    
    pub fn get_string(&self, key: &String) -> Option<String> {
        if let Some(s) = self.strings.get(key) {
            Some(s.clone())
        } else if let Some(parent) = self.parent.clone() {
            parent.borrow().get_string(key)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum TemplateNode {
    Page {
        path: String,
        front_matter: FrontMatter,
        content_node: Rc<TemplateNode>,
        output_path: PathBuf,
        parent: Option<Rc<TemplateNode>>,
    },
    Layout {
        name: String,
        front_matter: FrontMatter,
        content: String,
        parent: Option<Rc<TemplateNode>>,
    },
    IfBlock {
        condition: String,
        true_branch: Rc<TemplateNode>,
        false_branch: Option<Rc<TemplateNode>>,
    },
    ForEachBlock {
        key: String,
        item_name: String,
        body: Rc<TemplateNode>,
    },
    Func {
        name: String,
        args: Vec<String>,
        block_content: Option<String>,
    },
    StringContent(String),
    Composite(Vec<TemplateNode>),
}

impl TemplateNode {
    pub fn new_page(
        path: String,
        front_matter: FrontMatter,
        content_node: Rc<TemplateNode>,
        output_path: PathBuf,
        parent: Option<Rc<TemplateNode>>,
    ) -> Rc<Self> {
        Rc::new(Self::Page {
            path,
            front_matter,
            content_node,
            output_path,
            parent,
        })
    }
    
    pub fn new_layout(
        name: String,
        front_matter: FrontMatter,
        content: String,
        parent: Option<Rc<TemplateNode>>,
    ) -> Rc<Self> {
        Rc::new(Self::Layout {
            name,
            front_matter,
            content,
            parent,
        })
    }
    
    pub fn render(&self, context: Rc<RefCell<TemplateContext>>, global_context: &GlobalContext) -> String {
        match self {
            Self::Page { content_node, parent, front_matter, .. } => {
                // Create a new context for this page
                let page_context = TemplateContext::new(Some(context.clone()));
                
                // Add front matter to context
                page_context.borrow_mut().add_front_matter(front_matter);
                
                // Render the page content
                let content_output = content_node.render(page_context.clone(), global_context);
                
                // If there's a parent layout, render it with our content
                if let Some(parent) = parent {
                    // Create a new context for the layout
                    let layout_context = TemplateContext::new(Some(page_context));
                    
                    // Add the rendered content as "content" in the context
                    layout_context.borrow_mut().strings.insert("content".to_string(), content_output);
                    
                    // Render the parent layout
                    parent.render(layout_context, global_context)
                } else {
                    // No parent layout, just return the content
                    content_output
                }
            }
            Self::Layout { content, parent, front_matter, .. } => {
                // let content_node = parse_control_blocks(content, global_context);
                // let mut output = content_node.render(context.clone(), global_context);
                // // let mut output = content.clone();
                
                // if let Some(parent) = parent {
                //     let parent_output = parent.render(context.clone(), global_context);
                //     output = parent_output.replace("{{ content }}", &output);
                // }
                // output = Self::perform_substitutions_strings(output, front_matter);
                // self.apply_substitutions(&output, context, global_context)
                
                let mut output = content.clone();
                
                if let Some(parent) = parent {
                    let parent_output = parent.render(context.clone(), global_context);
                    output = parent_output.replace("{{ content }}", &output);
                }
                output = Self::perform_substitutions_strings(output, front_matter);
                output = self.apply_substitutions(&output, context.clone(), global_context);
                
                let content_node = parse_control_blocks(&output, global_context);
                content_node.render(context, global_context)
            }
            Self::IfBlock { condition, true_branch, false_branch } => {
                let ctx = context.borrow();
                if ctx.get_string(condition).is_some() {
                    true_branch.render(context.clone(), global_context)
                } else if let Some(false_branch) = false_branch {
                    false_branch.render(context.clone(), global_context)
                } else {
                    String::new()
                }
            }
            Self::ForEachBlock { key, item_name, body } => {
                let ctx = context.borrow();
                if let Some(Value::Sequence(items)) = ctx.json_data.get(key) {
                    items.iter()
                    .map(|item| {
                        let new_ctx = TemplateContext::new(Some(context.clone()));
                        if let Value::Mapping(map) = item {
                            for (k, v) in map {
                                if let (Some(k), Some(v)) = (k.as_str(), v.as_str()) {
                                    new_ctx.borrow_mut().strings.insert(k.to_string(), v.to_string());
                                }
                            }
                        }
                        body.render(new_ctx, global_context)
                    })
                    .collect()
                } else {
                    String::new()
                }
            }
            Self::Func { name, args, block_content } => {
                if let Some(func) = global_context.functions.get(name) {
                    func(args, block_content.as_deref(), context, global_context)
                } else {
                    name.clone()
                }
            }
            Self::StringContent(s) => s.clone(),
            Self::Composite(template_nodes) => {
                template_nodes.iter()
                .map(|x| x.render(context.clone(), global_context))
                .collect::<Vec<String>>()
                .join("")
            },
        }
    }
    
    fn perform_substitutions_str(s: String, k: &str, v: &str) -> String {
        let mut s = s.to_string();
        // spacing
        for k in vec![format!(" {} ", k), k.to_string()] {
            // formatting
            for k in vec![format!("{{{{{}}}}}", k), format!("{{{}}}", k)] {
                s = s.replace(&k, v);
            }
        }
        s
    }
    
    fn perform_substitutions_strings(s: String, strings: &HashMap<String, String>) -> String {
        let mut output = s.clone();
        for (key, value) in strings {
            output = Self::perform_substitutions_str(output, key, value);
        }
        output
    }
    
    fn apply_substitutions(&self, s: &str, context: Rc<RefCell<TemplateContext>>, global_context: &GlobalContext) -> String {
        let mut output = s.to_string();
        let ctx = context.borrow();
        
        output = Self::perform_substitutions_strings(output, &ctx.strings);
        
        // Node substitutions
        let rendered = ctx.nodes.iter()
        .map(|x| (x.0.clone(), x.1.render(context.clone(), global_context)))
        .collect::<HashMap<String, String>>();
        output = Self::perform_substitutions_strings(output, &rendered);
        
        // do with parent context
        if let Some(parent) = context.borrow().parent.clone() {
            self.apply_substitutions(&output, parent.clone(), global_context)
        } else {
            // apply root string substitutions
            for (key, value) in &global_context.site_strings {
                output = Self::perform_substitutions_str(output, key, value);
            }
            output
        }
    }
    
    fn render_json_list(&self, key: &str, context: Rc<RefCell<TemplateContext>>, _: &GlobalContext) -> String {
        context.borrow().json_data.get(key)
        .and_then(|data| data.as_sequence())
        .map(|items| {
            items.iter().filter_map(|item| item.as_mapping()).fold(
                String::from("<ul>\n"),
                |mut output, obj| {
                    output.push_str("<li>");
                    if let Some(Value::String(title)) = obj.get("title") {
                        output.push_str(&format!("<h3>{}</h3>", title));
                    }
                    if let Some(Value::String(desc)) = obj.get("description") {
                        output.push_str(&format!("<p>{}</p>", desc));
                    }
                    output.push_str("</li>\n");
                    output
                },
            ) + "</ul>"
        })
        .unwrap_or_default()
    }
    
    pub fn print_tree(&self, indent: usize) {
        match self {
            Self::Page { path, parent, .. } => {
                println!("{:indent$}📄 {} (Page)", "", path, indent = indent);
                if let Some(parent) = parent {
                    parent.print_tree(indent + 2);
                }
            }
            Self::Layout { name, parent, .. } => {
                println!("{:indent$}📦 {} (Layout)", "", name, indent = indent);
                if let Some(parent) = parent {
                    parent.print_tree(indent + 2);
                }
            }
            Self::IfBlock { condition, true_branch, false_branch } => todo!(),
            Self::ForEachBlock { key, item_name, body } => todo!(),
            Self::Func { name, args, block_content } => todo!(),
            Self::StringContent(_) => todo!(),
            Self::Composite(template_nodes) => todo!(),
        }
    }
}

fn parse_control_blocks(content: &str, global_context: &GlobalContext) -> Rc<TemplateNode> {
    let mut nodes = Vec::new();
    let mut remaining = content;
    
    while let Some(open_pos) = remaining.find("{{") {
        let before = &remaining[..open_pos];
        if !before.is_empty() {
            nodes.push(TemplateNode::StringContent(before.to_string()));
        }
        
        let close_pos = remaining[open_pos..].find("}}").unwrap() + open_pos;
        let tag = &remaining[open_pos+2..close_pos].trim();
        remaining = &remaining[close_pos+2..];
        
        match tag.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["if", condition] => {
                let (inner_content, new_remaining) = parse_block_content(remaining, "endif");
                remaining = new_remaining;
                
                // Split into if and else parts if needed
                let (true_content, false_content) = match inner_content.split_once("{{ else }}") {
                    Some((true_part, false_part)) => (true_part, Some(false_part)),
                    None => (inner_content, None),
                };
                
                let true_node = parse_control_blocks(true_content, global_context);
                let false_node = false_content.map(|c| parse_control_blocks(c, global_context));
                
                nodes.push(TemplateNode::IfBlock {
                    condition: condition.to_string(),
                    true_branch: true_node,
                    false_branch: false_node,
                });
            },
            ["else"] => {
                // Only warn if this isn't part of a string that looks like a real else
                if !tag.starts_with("else ") && !tag.ends_with(" else") {
                    eprintln!("Warning: found else without matching if in content: {:?}", tag);
                }
                // Skip this token and continue parsing
            },
            ["foreach", key, "as", item_name] => {
                let (inner_content, new_remaining) = parse_block_content(remaining, "endforeach");
                remaining = new_remaining;
                let inner_node = parse_control_blocks(inner_content, global_context);
                nodes.push(TemplateNode::ForEachBlock {
                    key: key.to_string(),
                    item_name: item_name.to_string(),
                    body: inner_node,
                });
            },
            _ => {

            }
        }
    }
    
    if !remaining.is_empty() {
        nodes.push(TemplateNode::StringContent(remaining.to_string()));
    }
    
    Rc::new(TemplateNode::Composite(nodes))
}

fn parse_block_content<'a>(content: &'a str, end_tag: &'a str) -> (&'a str, &'a str) {
    let end_pattern = format!("{{{{ {end_tag} }}}}");
    let end_pos = content.find(&end_pattern).unwrap_or(content.len());
    let args_start = end_pos + end_pattern.len();
    if args_start < content.len() {
        (&content[..end_pos], &content[args_start..])
    } else {
        (&content[..end_pos], "")
    }
}

struct GlobalContext {
    output_base: String,
    layout_cache: HashMap<String, Rc<TemplateNode>>,
    site_strings: HashMap<String, String>,
    functions: HashMap<String, Rc<dyn Fn(&[String], Option<&str>, Rc<RefCell<TemplateContext>>, &GlobalContext) -> String>>,
}

impl GlobalContext {
    pub fn new(output_base: &str) -> Self {
        let mut functions = HashMap::new();
        
        // Register built-in functions
        // functions.insert("uppercase".to_string(), Rc::new(|args, _, _, _| {
        //     args.get(0).map(|s| s.to_uppercase()).unwrap_or_default()
        // }));
        
        // functions.insert("lowercase".to_string(), Rc::new(|args, _, _, _| {
        //     args.get(0).map(|s| s.to_lowercase()).unwrap_or_default()
        // }));
        
        Self {
            output_base: output_base.to_string(),
            layout_cache: HashMap::new(),
            site_strings: HashMap::new(),
            functions,
        }
    }
    
    pub fn get_layout(&mut self, name: &str) -> Rc<TemplateNode> {
        if let Some(layout) = self.layout_cache.get(name) {
            return layout.clone();
        }
        
        let path = Path::new("templates/layouts").join(format!("{}.tpl", name));
        let content = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read template: {}", name));
        
        let (front_matter, html) = parse_front_matter(&content);
        let front_matter = parse_yaml_front_matter(front_matter).unwrap_or_default();
        
        // Check if this layout has a parent layout
        let parent_layout = if name != "site" && name != "default" {
            Some(self.get_layout("default"))
        } else if name == "default" {
            Some(self.get_layout("site"))
        } else {
            None
        };
        
        let layout = TemplateNode::new_layout(name.to_string(), front_matter, html.to_string(), parent_layout);
        self.layout_cache.insert(name.to_string(), layout.clone());
        layout
    }
    
    pub fn load_site_data(&mut self) {
        let path = std::env::current_dir().unwrap().join("data/site.yaml");
        let path = path.to_str().unwrap();
        let site_yaml = load_json_data(path).expect("could not get data/site.yaml");
        if let Value::Mapping(mapping) = site_yaml {
            self.load_site_data_from_yaml_mapping(mapping)
        } else {
            panic!("unsupported site yaml type")
        }
    }
    
    pub fn load_site_data_from_yaml_mapping(&mut self, mapping: serde_yaml::Mapping) {
        for (k, v) in mapping.iter() {
            let v = if v.is_bool() {
                v.as_bool().unwrap().to_string()
            } else {
                v.as_str().unwrap().to_string()
            };
            self.site_strings.insert(k.as_str().unwrap().to_string(), v);
        }
    }
}

fn build_page(
    path: &str,
    global_context: &mut GlobalContext,
) -> Result<Rc<TemplateNode>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let (front_matter, markdown) = parse_front_matter(&content);
    let mut front_matter = parse_yaml_front_matter(front_matter).unwrap_or_default();
    
    // Set defaults
    if !front_matter.contains_key("layout") {
        front_matter.insert("layout".to_string(), "default".to_string());
    }
    if !front_matter.contains_key("title") {
        front_matter.insert("title".to_string(), 
        Path::new(path).file_stem().unwrap().to_string_lossy().into_owned());
    }
    
    // Convert markdown to HTML
    let mut html_content = String::new();
    html::push_html(&mut html_content, Parser::new_ext(markdown, Options::all()));
    
    // Parse control blocks in the content
    let content_node = parse_control_blocks(&html_content, global_context);
    
    // Create output path
    let output_path = PathBuf::from(&global_context.output_base)
    .join(file_path_stem(Path::new("."), path))
    .with_extension("html");
    
    // Get the layout hierarchy
    let layout_name = front_matter.get("layout").unwrap();
    let layout = global_context.get_layout(layout_name);
    
    // Create the page with the layout as parent
    Ok(TemplateNode::new_page(
        path.to_string(),
        front_matter,
        content_node,
        output_path,
        Some(layout),
    ))
}

fn get_formatted_datetime() -> String {
    let now_system_time = SystemTime::now(); // Get the current system time
    let now_local_datetime: DateTime<Local> = DateTime::from(now_system_time); // Convert to local DateTime
    now_local_datetime.format("%c").to_string()
}

// ========== Helper Functions ==========

fn parse_front_matter(content: &str) -> (&str, &str) {
    if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            (parts[1].trim(), parts[2].trim())
        } else {
            ("", content)
        }
    } else {
        ("", content)
    }
}

fn parse_yaml_front_matter(front_matter: &str) -> Option<FrontMatter> {
    if front_matter.is_empty() {
        None
    } else {
        serde_yaml::from_str(front_matter).ok()
    }
}

fn get_md_files_recursive(path: &Path) -> Vec<String> {
    fs::read_dir(path).ok()
    .map(|entries| {
        entries.filter_map(|entry| entry.ok())
        .flat_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                get_md_files_recursive(&path)
            } else if path.extension().map_or(false, |ext| ext == "md") {
                path.to_str().map(|s| s.to_string()).into_iter().collect()
            } else {
                Vec::new()
            }
        })
        .collect()
    })
    .unwrap_or_default()
}

fn file_path_stem(base_path: &Path, full_path: &str) -> String {
    Path::new(full_path).strip_prefix(base_path)
    .map(|p| p.to_string_lossy().into_owned())
    .unwrap_or_else(|_| full_path.to_string())
}

fn copy_assets(src: &str, dst: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(src).exists() {
        return Ok(());
    }
    
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = Path::new(dst).join(entry.file_name());
        
        if path.is_dir() {
            copy_assets(path.to_str().unwrap(), dest_path.to_str().unwrap())?;
        } else {
            fs::copy(path, dest_path)?;
        }
    }
    Ok(())
}

fn load_json_data(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    Ok(serde_yaml::from_reader(file)?)
}

// ========== Main Function ==========

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_base = "output";
    let mut global_context = GlobalContext::new(output_base);
    global_context.load_site_data();
    fs::create_dir_all(output_base)?;
    
    // Build and render all pages
    for path in get_md_files_recursive(Path::new(".")) {
        if !path.contains("/assets/") && !path.contains("assets/") {
            if let Ok(page) = build_page(&path, &mut global_context) {
                // Print the tree structure
                println!("Template Tree Structure:");
                page.print_tree(0);
                
                // Render the page
                if let TemplateNode::Page { output_path, front_matter, .. } = &*page {
                    // Create context
                    let ctx = TemplateContext::new(None);
                    ctx.borrow_mut().strings.insert("page.date".to_string(), get_formatted_datetime());
                    ctx.borrow_mut().add_front_matter(&front_matter);
                    
                    // // Handle JSON data
                    // if let Some(json_path) = front_matter.get("json_data") {
                    //     if let Ok(json_data) = load_json_data(json_path) {
                    //         context.borrow_mut().json_data.insert("items".to_string(), json_data);
                    //         context.borrow_mut().nodes.insert(
                    //             "json_list".to_string(),
                    //             Rc::new(TemplateNode::JsonList("items".to_string())),
                    //         );
                    //     }
                    // }
                    
                    let html = page.render(ctx, &global_context);
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(output_path, html)?;
                }
            }
        }
    }
    
    copy_assets("assets", &format!("{}/assets", output_base))?;
    println!("Site generation complete!");
    Ok(())
}