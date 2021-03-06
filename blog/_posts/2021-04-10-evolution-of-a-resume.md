---
date: 2021-04-11
title: Evolution of a Resume
image: /assets/img/blog/test.jpg
description: >
  How my resume started, what forms it took, and why is it on GitHub Pages now (and why yours should be too)
tags: [career, resume, jekyll]
category: [blog, projects]
layout: post
---

- very limited much server side logic (a 'static' site is generated from data)
- < partial > is converted to {{ content }}
- Markdown (.md) is like a mix between view data (yaml) and view style / internal structure
- .html template files are like the layout and partial views (https://jekyllrb.com/docs/includes/#including-files-relative-to-another-file)

Reasons for doing this:
- Cut down on hosting costs (domain, server hardware, electricity)
- Cut down on maintenence costs (static sites are less prone to unexpected runtime behavior)
- Shift over to a streamlined development pipeline, from idea to release as a web page
- Lots of public examples, documentation, and resources (even plugins for common tasks)
- Make a site's code public without worrying about particular security risks (there's no endpoint to pass SQL injection scripts to)
- 