---
date: 2026-01-01
title: Adult Toy Store - Client Request
image: /assets/images/projects/toy-store-screens.png
image_alt: "3 different slices of screenshots of the toy store website"
description: >
    My friend discussed with me making a site for her toy store and with my web app and computer graphics knowledge, I created a design-your-own-toy online store.
tags: [ startup, computer-graphics, rust ]
categories: projects
layout: post
---


My friend makes toys and discussed with me her plans for an online store and the different ways people can customize and design their toys. By design we mean the visual styling, size, and certain physical attributes of the toy, not the actual morphology of the toy. The user is able to choose from an array of different models of toy as a base for visual designs. The final price of the toy depends on the complexity and amount of materials and time required to make it. This is a fairly simple premise other than an online toy viewer or designer that does not leak the model data. There are existing methods for extracting a 3D model from a collection of images or videos, but there are also ways to mitigate easy theft. Experts will find ways around the model restrictions so spending extra time on extra mitigation methods seemed not worth the time.

## Tech Stack
Rust, HTML, WebGL for client-side rendering, OpenGL for server-side rendering.

## Project Requirements
- Point of Sale access (concurrently in-use phone apps, ideally)
- Online store (Carts, Cart Items, Products, Product Variants, Product Styles, Orders)
- Online toy designer (3D view without leaking the model data)
- Back of house (Customers, Employees, metrics)
- Log of transactions for audits
- Performant and efficient, ideally run on a local machine
- Various views of the data and toys for promotional content, marketing, sales

## Project Development
- **Version 0 - CLI and Server Basic Store**
    
    Very basic store server with a shared API with the CLI client for managing carts, cart items, products, variants, orders, customers, employees, and metrics.

- **Version 1 - Website and POS app**

    Building off of V.0, I added a webview to the server which was a lot like the CLI but in a browser and a much better experience. The POS app is a laptop, tablet, or mobile view that behaves as an expected POS tool would, including scanning product barcodes. The barcode goes through a few different methods of scanning in case one algorithm fails to read it. A usb webcam and browser supported scanner are also supported. Added more ways to style toys.

- **Version 2 - Rendering Updates**

    The toys on the public view of the website are now GIFs of a 360 degree rotation of the toy with the selected style and options. The toy renders are cached and tracked by usage. The renders are done in the background so that the web server process can do other tasks while the render server handles render requests. The rendering server is distributable and pooled, so you can start multiple rendering servers and if one crashes, the whole rendering system does not fail. The system is overall more efficient and performant while being more secure.

## Architecture
(todo)


## Reception
Received well but my friend decided to work with her partner who is also a software developer and more in the loop as they are married and live together. They also delegated some part of the software development to AI agents that can run 24/7 on developing the system and they can focus on making toys.

## Lessons Learned
Clearly document project requirements and client requests, and setup regular weekly check-ins in order to remain in the loop on everything. Consider using larger existing solutions that allow extensions or mods instead of custom built solutions. AI agents might be able to write code 24/7 but it doesn't always write code as expected or intended and is not as reliable or capable as a human who writes code and takes breaks.