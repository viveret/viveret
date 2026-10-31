---
date: 2016-12-01
title: McKinstry - Project Spider Silk
description: >
  I worked on one large overarching project that contained many connected subprojects
tags: [ mckinstry, c#, vb ]
categories: [ projects ]
layout: post
---

We picked the name "Project Spider Silk" because of how much it connected to other applications and how it connected them all to each other in a web. The name is merely a synonym and alias for the collection of projects but it solved the problem of data silos and friction in working with data. As more connections were made, more possibilities were created, which shaped how the project was built, how it functioned, and overall improved how connected engineers felt to coworkers and the data they were working on.

## Excel Spreadsheets Plugin
The Excel Spreadsheets plugin would sync data in the Excel documents with the central system. When the user opens an Excel document that the plugin recognizes, it will check the central system first for new data and ask the user if they want to update their version or cherry pick changes. Some of the Excel documents were necessary to complete and send results back to the central system as a part of the engineering design process on some projects.

## Revit Plugin

### What is the Revit Plugin?
> Revit is a **Building Information Modeling (BIM)** software application developed by Autodesk. Unlike traditional Computer-Aided Design (CAD) software, which primarily focuses on creating 2D drawings and 3D models as geometric representations, Revit is fundamentally a tool for creating **intelligent, parametric models** specifically for the Architecture, Engineering, and Construction (AEC) industry.

My plugin (if finished) would have synced data between Revit models and the central system. 

## Program Orchestration
Some programs are used in the exact same way repeatedly hundreds or thousands of times, and having our plugin in the application or available to the user to run to orchestrate things helped accomplish the task of automation. This included more than syncing data, but I cannot mention too many of the details other than it related to building data, modeling, and otherwise computationally-heavy calculations.

## Virtual Environment Program Orchestration
Certain applications require only one instance is running on a machine at a time. In order to run these concurrently on one machine, the application needs to be in a virtual machine. When the application is in a virtual machine, we can interact with it similarly to as if it were running in the same operating space - we can send key and mouse commands, take screenshots, perform either image difference, text extraction via clipboard or OCR, and interact with the app like an API. To make it easier and more reliable to use we built a server and load distributor. 

## Smart Document Generator (Docs Plugin)
The centralized system can coordinate with document templates to generate DOCX, PDF, and MarkDown files. These are typically for external rather than internal use, so the templates contain company styling and follow external publishing guidelines. There is also a WYSIWYG web editor for document generation that helps the user connect to the data.

## Parallel Coordinates Chart
One problem the engineers and sales had was with generating data outputs for models from a range of parameters that can be adjusted on a spectrum. This type of graph or interface is called a "parallel coordinates chart."
> **What is a "parallel coordinates chart"?**
> 
> 
> A Parallel Coordinates Chart (also known as a Parallel Plot or Parallel Coordinates Plot) is a graphical method for visualizing data with multiple variables (dimensions). Instead of plotting data points on a standard 2D plane with axes at right angles, this chart plots each variable along its own distinct axis, all drawn parallel to each other.

The data is usually generated before-hand as the amount of data in the dataset for a parallel coordinates chart tends to be quite large. There are also different ways to use the chart, but the main general use case is for understanding relationships in large amounts of data. It can be used to make final or guiding decisions, as the dataset can be updated to make use of insights gained from previous iterations of the chart and dataset. The selection ranges or values can also be fed back into the central system to update documents and models.

## Automated Testing
I wrote a fair amount of tests in order to make sure any changes to such a large system did not cause regressions or destructive problems. Some small bugs were permitted as it was a research and development project, but overall I didn't want a single link, button, option, deployment, or system possibility to not work as expected. Sample test data came from test datasets as well as readonly production data.

## Continuous Deployment
Whenever a pull request or change was approved and merged, it would go through all the tests then be deployed to the server. Plugins for Excel, Word, Revit would update automatically from the server. I think eventually we created a staging and dev environment so that we could deploy code and not worry about affecting production.

## Server / Web App Architecture
- HTTP and HTML (mostly handled by MVC)
- API and Controllers/Actions
- Services, Repositories, background services
- ORM (map between entities and data, data mappings to entities)
- Constant values, formulas, codes and standards of values

> **What is an ORM?**
>
>
> An ORM (Object-Relational Mapper) is a programming tool or database abstraction layer that mediates between object-oriented programming languages (like Python, Java, C#, JavaScript/TypeScript) and databases (like PostgreSQL, MySQL, SQLite, SQL Server, Oracle, etc.). It's designed to overcome the mismatch between object-oriented code and relational databases.


## Excel Plugin Architecture
There are 2 paths for the plugin, either it can be a known type of spreadsheet workbook that has a specific structure, or an unknown generic type of workbook that has a spreadsheet for the local Excel "database". 

### Known type of workbook:
- Has specific sheets with required names and values
- No generic DB sheet to sync data - the data comes from the workbook sheets directly

### Unknown generic workbook:
- Single sheet dedicated to syncing data between the workbook and server
- Other sheets reference the data sheet cells and ranges