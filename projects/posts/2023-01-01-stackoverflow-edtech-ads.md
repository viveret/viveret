---
date: 2023-01-17
title: Stack Overflow Jobs - Ed Tech Ads
image: /assets/images/ed-tech-ads.png
image_alt: "Screenshot of different variants of ed tech ads"
description: >
  Educational technology course ads I worked on at Stack Overflow
tags: [ stackoverflow, c# ]
categories: [ projects ]
layout: post
---

Ed Tech Ads was a short lived trial feature for advertisements of educational courses and classes relevant to either the current question in view, the user's browsing history, or a random selection from curated items. Data was sent to us, we analyzed the data and matched them to questions, and fulfilled a quota of required views while tracking conversions and return on investment. There was an option to do profit sharing instead of a traditional audience size and duration purchase as well as compare efficacy of the different visual styles.

## Tech Stack
C# and ASP.NET, SQL, Redis, Machine Learning (handled by data team)

## Features
- Different visual styles of ad
- Multiple data sources with failover
- Tracking return on investment through ad metric data


## Excerpts from Official Announcement (Jan 17th, 2023)
[Link to the Stack Overflow announcement post](https://meta.stackoverflow.com/questions/422723/we-re-bringing-advertisements-for-technology-courses-to-stack-overflow)

> Beginning in late January, we’re partnering with two online course providers to present sponsored technology course recommendation advertisements on Stack Overflow. This is a pilot study for this type of ad, and we’ll be running tests over the course of a few months to develop an understanding of the value that course recommendations add to the platform.
> On the day we launch, you will begin to see course recommendations from our first two partners - Pluralsight and Udemy.

> When we show you these ads, we will display them in the existing sidebar ad slot, rather than create an additional ad slot dedicated to course recommendations. Our primary aim is to improve the usefulness and relevance of our ad slots to users.
> We will base the ads on your Stack Overflow browsing history, only if you have opted in to tracking.

> The pilot will last approximately four months, and we'll be running experiments (e.g. altering the layout or design of the ad) during this time. We plan to partner only with Pluralsight and Udemy for the duration of the pilot study. Based on the results of the pilot study and your feedback, we'll decide on next steps. That includes a determination as to when we will open this ad slot up to additional partners that create technology courses relevant to Stack Overflow (provided they meet our advertiser guidelines).

## Architecture
1. Data team receives data from ed tech course partners and does analysis, building a model that can match course data to questions and accepted answers.
2. Ads team builds a new ad type and wires ad selection into the ed tech course selection algorithm.
3. Ads team builds the view and metrics tracking.
4. Data team updates models and matching data.

## Lessons Learned
The ed tech ads were a really good idea - connect people trying to solve problems with content about technologies they might be having trouble using or want to learn more about. I think we should have allowed user generated content by forwarding users to the sites we partnered with the source ed tech ads from, and the revenue model could have in my opinion been better - pay for a certain number of views.