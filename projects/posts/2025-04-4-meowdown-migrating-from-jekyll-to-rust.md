---
date: 2025-04-4
title: MeowDown - Migrating from Jekyll to Rust
description: >
  I took my static jekyll site and wrote a Rust static site generator
tags: [ personal, jekyll, rust ]
category: projects
layout: post
---

[Link to MeowDown.](https://github.com/viveret/meowdown)

I was not familiar with Jekyll and not satisfied with the site build/deployment, so I started ideating on ways to improve how my site was being generated. Rust already has a lot of libraries I could use to make it easier to build and understand how it works. I decided to make a CLI app for building markdown files with a YAML header and in a folder of HTML/file templates and website assets into a bunch of static HTML files and website assets that can be deployed to any static site hosting like github pages.

## Tech Stack
Rust, templating library, HTML and JS

## Architecture of MeowDown

## My Site 

## Lessons Learned
I love being able to modify any part of my website's build process and that it primarily uses Rust and Markdown with HTML templates.