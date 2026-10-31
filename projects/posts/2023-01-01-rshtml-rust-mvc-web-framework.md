---
date: 2023-09-01
title: Rust MVC Web Framework (rshtml)
description: >
  A project that I used to learn more about the MVC framework
tags: [ personal, rust ]
categories: projects
layout: post
---

[Link to rshtml](https://github.com/viveret/rshtml)

I was curious about how some parts of the C# typed service dependency resolution and web hosting/routing worked. I did some research but not a lot on the different available web frameworks for rust. I mostly wanted to understand what the C# .net framework was doing in order to serve requests to middleware and how it knew to map requests to specific controllers and actions. I did end up understanding how it works and how simple but effective it is and how it actually can be applied to a lot of different software applications.

## Architecture
- Service Collection and dependency injection system
- Request Middleware, request and response handlers
- Filters, authentication, controllers, actions, views
- Partial views, API routes, client event mapping
- Razor like syntax for view templates
- Macros for auto-generating code

[For more information on this project, see the repository here](https://github.com/viveret/rshtml)

## Lessons Learned
I now understand the difference between a minimal file hosting server, a proxy server like httpd, and a web server with MVC. My PHP projects used a different an in my opinion "more spaghetti-like" way of resolving dependencies in all the examples and resources I was using to learn. A service collection with clear scopes helps reduce a lot of the complexity of building a web app with many layers.

Another lesson learned is that although the flexibility that can come with additional complexity is worth it for some problems, but some problems are solvable with less complex and less flexible technology that is more dependable and easier to learn and use. 