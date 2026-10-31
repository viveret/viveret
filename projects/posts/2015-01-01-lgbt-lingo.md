---
date: 2015-01-01
title: LGBT Lingo - My First App
image: /assets/images/projects/lgbt-lingo-screens.png
image_alt: "A few screenshots of the app, the first is an index, the second is a guide screen, and the third screen is a word definition"
description: >
  A project that I used to learn about Android software development as well as make something meaningful and helpful to myself and the LGBT community
tags: [ educational, personal ]
categories: projects
layout: post
---

[Link to LGBT Lingo.](https://viveret.github.io/lgbt-lingo/)

When I was learning more about releasing software for one of the more common devices, phones, and learning more about gender and sexuality, and all the different terms for different expressions and identities, I had an idea that it would be helpful to have an easily searchable list of LGBTQIA+ related words and their definitions. This would both help people looking to find the right words for their feelings, and help those hearing or reading new words understand what others are expressing.

## Tech Stack
- 2014 - 2019: Java, Android SDK, Google Play Store
- 2016 - 2019: C# .NET core, my home server / personal site
- 2019 - now: Rust, HTML, JS, my Static Site Generator meowdown, GitHub (host)

## App Flow
The user is first shown an alphabetically sorted list of words and short definitions. Above the list is a search bar and to the right is an index of A-Z letters to jump to in the list. When a user clicks on a word/definition row, they are brought to a view of the word with a longer definition, the source of the definition, and more context around the word, like if it is a slur. There is also a button to favorite the word. The user can go back to the main list and see that there is a button to filter the list by favorites. The app also has a menu button for app information that describes the purpose of the app and how to contribute to the words or code.

Initially the words/dictionary data were packaged with the app, but later I changed it to sync with the GitHub repo's version of the file if it is newer.

I was not able to keep the app up to date with the many iterations of Android SDK and Google Play Store updates, so I let the app get unlisted around 2019 to focus on the website version which can run as a web app.

## Website Flow
The website works exactly the same as the app, but uses different technologies and run on a different platform. Favorites are removed because the browser already has a bookmarking system. The dictionary data comes from the same source as the site and there are no issues or delays with updating the repo.

## Reception
Most of the feedback was good and repeated the same ideas that motivated me to make the app. Some feedback was earnest and pertained to the accuracy of definitions or what should be added, but a sizable amount of the feedback was not genuine and was unfortunately hateful. I also noticed later on sometime after 2021 that transphobes and homophobes were using screenshots of my app to disparage and spread hatred and fear. I believe it is a human right to understand one's self and be able to express how one identifies as accurately as they feel the need to.