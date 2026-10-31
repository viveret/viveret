---
date: 2019-08-01
title: Stack Overflow Jobs - Candidate Message Search
image: /assets/images/projects/candidate-messages-pseudo-screen.png
image_alt: "Wireframe drawing of the candidate message search"
description: >
  More details about the candidate message search engine I made at Stack Overflow
tags: [ stackoverflow, c# ]
categories: [ projects ]
layout: post
---

The first few weeks to month at Stack Overflow for new employees is spent brainstorming, researching, planning, developing, and presenting a problem and solution or new feature. I noticed the messages page between recruiters, hiring managers, and candidates only showed a paginated view without any filters or options. I just finished learning about Elastic Search and had already been messing around with sentiment analysis and machine learning for a while, so I came up with the idea and plan to implement candidate message search, and recruiters and hiring managers can search by sentiment.

## Tech Stack
C#, Elastic Search, tiny-dnn for the sentiment analysis ML model

## Architecture
- Elastic index: UserMessages (a message for either candidate or employer) with extra options for searching
- Background service in the web server to sync the messages database with an elasticsearch index specifically optimized for searching messages and by sentiment
- One time training of a machine learning model on movie reviews (the idea is negative reviews indicate negative sentiment, positive reviews indicate positive sentiment, and reviews are easy datasets to find or generate)
- Background service to run the machine learning model on the elastic search index of messages to be able to filter by sentiment
- UI and API changes for interfacing the user with this new feature

## Reception
The project was one of the clearest cases of a project that had a small scope, sizable impact, and could be viable as a released feature, and was used as an example project for new employees after me. However, we did not go through with the process of turning the demo project into a real feature because there were existing projects the team was working on that had a higher and clearer return on investment.

## Lessons Learned
Despite it being a feature which solved a repeated customer complaint, it was not graduated into a real work item to be implemented and deployed. The main issue is duplicating the messages into another service and making sure data is not exposed and is deleted when needed. Another issue with the sentiment search is the data is run through a model and this processing is not included in any documentation. For my demo I used sample data and not real user data.