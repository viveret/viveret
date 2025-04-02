use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;


use pulldown_cmark::Parser;
use pulldown_cmark::Options;
use pulldown_cmark::html;


#[derive(Debug, Clone, PartialEq)]
enum TemplateNode {
    Func(fn(&TemplateContext) -> String),
    String(String),
    Composite(Vec<TemplateNode>),
    Page(Page),
}

#[derive(Debug, Default)]
struct TemplateContext {
    pub strings: HashMap<String, String>,
    pub nodes: HashMap<String, TemplateNode>,
    pub parent: Option<Rc<TemplateContext>>,
}

impl TemplateContext {
    pub fn new(parent: Rc<TemplateContext>) -> Self {
        Self {
            strings: HashMap::new(),
            nodes: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn new_root() -> Self {
        Self {
            strings: HashMap::new(),
            nodes: HashMap::new(),
            parent: None,
        }
    }
}

impl TemplateNode {
    pub fn render(&self, context: &mut TemplateContext, global_context: &mut GlobalContext) -> String {
        match self {
            Self::Func(f) => f(context),
            Self::String(s) => {
                let mut output = s.clone();
                // Replace string placeholders
                for (key, value) in &context.strings {
                    output = output.replace(&format!("{{{}}}", key), value);
                }
                // Replace node references
                for (key, node) in context.nodes.clone() {
                    if node != *self {
                        output = output.replace(
                                        &format!("{{#{}}}", key),
                                        &node.render(context, global_context)
                        );
                    }
                }
                output
            }
            Self::Composite(nodes) => {
                nodes.iter().map(|node| node.render(context, global_context)).collect()
            }
            Self::Page(pg) => {
                pg.content.clone()
            }
        }
    }

    
    // Helper constructors
    pub fn str(s: &str) -> Self {
        Self::String(s.to_string())
    }

    pub fn func(f: fn(&TemplateContext) -> String) -> Self {
        Self::Func(f)
    }

    pub fn composite(nodes: Vec<TemplateNode>) -> Self {
        Self::Composite(nodes)
    }

    pub fn page(page: Page) -> Self {
	    Self::Page(page)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
struct FrontMatter {
    title: String,
    layout: Option<String>,
    // Add other front matter fields as needed
}

#[derive(Clone, Debug, PartialEq)]
struct Page {
    path: String,
    front_matter: FrontMatter,
    content: String,
    html_content: String,
    output_path: PathBuf,
    layout_template: Option<Rc<TemplateNode>>,
}

struct GlobalContext {
    pub output_base: &'static str,
    pub layout_templates: HashMap<String, Rc<TemplateNode>>
}

impl GlobalContext {
    pub fn new(
        output_base: &'static str
    ) -> Self { Self { output_base, layout_templates: HashMap::new() } }
}


fn get_or_load_layout_template(name: &String, g_ctx: &mut GlobalContext) -> Rc<TemplateNode> {
    if let Some(v) = g_ctx.layout_templates.get(name) {
	    v.clone()
    } else {
        let page = process_html_file(name, g_ctx);
        let x = Rc::new(TemplateNode::Page(page));
        g_ctx.layout_templates.insert(name.to_string(), x.clone());
        x
    }
}

fn process_html_file(
    path: &String,
    global_context: &mut GlobalContext
) -> Page {
    println!("processing html file {}", path);
    // rest of implementation   
}

fn process_markdown_file(
    path: &String,
    global_context: &mut GlobalContext,
) -> Result<Option<Page>, Box<dyn std::error::Error>> {
    println!("processing markdown file {}", path);
    
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Split front matter and markdown content
    println!("splitting");
    let (front_matter, markdown) = if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            (parts[1].trim(), parts[2].trim())
        } else {
            ("", content.as_str())
        }
    } else {
        ("", content.as_str())
    };

    // Parse front matter
    println!("parsing front matter");
    let front_matter: FrontMatter = if !front_matter.is_empty() {
        serde_yaml::from_str(front_matter)?
    } else {
        FrontMatter {
            title: path.clone(),
            layout: None,
        }
    };

    println!("front matter: {:?}", front_matter);

    // get layout template
    let layout_template = front_matter.layout.clone().map(|name| get_or_load_layout_template(&name, global_context));

    // Convert markdown to HTML
    let parser = Parser::new_ext(markdown, Options::all());
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    // Determine output path
    let cur_dir = std::env::current_dir().unwrap();
    let cur_dir = cur_dir.as_path();
    let stem_path = file_path_stem(cur_dir, path);
    let output_path = cur_dir.join(global_context.output_base).join(
        stem_path.to_string() + ".html",
    );
    println!("stem: {}, out: {}", stem_path, output_path.as_path().to_str().unwrap());

    Ok(Some(Page {
	    path: path.clone(),
        front_matter,
        content: markdown.to_string(),
        html_content,
        output_path,
	    layout_template,
    }))
}


fn render_page(
    page: &Page,
    global_context: &mut GlobalContext,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("render_page {}", page.path);

    // Create parent directory if needed
    if let Some(parent) = page.output_path.parent() {
        println!("create parent {}", parent.to_str().unwrap());
        fs::create_dir_all(parent)?;
    }     

    let html_output = if let Some(layout_template) = page.layout_template.clone() {
        let mut ctx = TemplateContext::new_root();
        ctx.nodes.insert("body".to_string(), TemplateNode::Page(page.clone()));
	    layout_template.render(&mut ctx, global_context)
    } else {
	    page.html_content.clone()
    };
    
    println!("writing to {}", page.output_path.as_path().to_str().unwrap());
    let mut file = File::create(&page.output_path)?;
    file.write_all(html_output.as_bytes())?;

    Ok(())
}

fn get_md_files_recursive(path: &Path) -> Vec<String> {
    let mut md_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
		// println!("looking at {:?}", path);
                if path.is_dir() {
                    md_files.extend(get_md_files_recursive(&path));
                } else if let Some(ext) = path.extension() {
                    if ext == "md" {
                        if let Some(path_str) = path.to_str() {
                            md_files.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }
    
    md_files
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory
    let mut global_context = GlobalContext::new("output");
    fs::create_dir_all(global_context.output_base)?;

    let cur_dir = std::env::current_dir().unwrap();
    let cur_dir = cur_dir.as_path();
    let files = get_md_files_recursive(cur_dir);
    println!("found {} md files", files.len());
    for path in files.iter() {
        let stem = file_path_stem(cur_dir, path);
        if stem.starts_with("/assets/") || stem.starts_with("assets/") {
            continue;
        }
	
	    if let Some(page) = process_markdown_file(path, &mut global_context)? {
            render_page(&page, &mut global_context)?;
        }
    }

    Ok(())
}


fn file_path_stem<'a>(stem: &'a Path, path: &'a str) -> &'a str {
    let stem = stem.to_str().unwrap();
    if path.contains(stem) {
        let mut path_parts = path.split(stem);
        path_parts.next();
        if let Some(rest) = path_parts.next() {
            return &rest[1..];
        }
    }
    return "";
}
