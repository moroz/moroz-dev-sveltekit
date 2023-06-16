---
title: "Learning iOS and Swift. Day 27: Login view"
date: 2022-06-14
slug: learning-ios-and-swift-day-twenty-seven
lang: en-US

summary: |
  Today I implemented a login view with a mocked authentication step.

---

Following the excellent [SwiftUI Login Screen Workflow](https://www.youtube.com/watch?v=QrTChgzseVk) tutorial, I built a very basic login form that simulates an API request, and signs the user in if the password is correct.
I am using more and more singleton classes. It seems that for classes that do not need state and reactivity, singleton classes are the way to go for most everyday tasks.
My original instinct has been to implement all methods using static methods, but this makes the syntax more and more convoluted.
With singleton classes, I can write all properties and methods using instance methods, and they get exposed on the singleton instance.

The next step for the development of the current app will be actually performing the mutation.
I still don't understand what an `@escaping` closure is, how `completion()` works, and how to eloquently write asynchronous functions that accept success callbacks.
I will explore these topics in the following days, while I connect my current project to an API.
Luckily, the API for the service I am connecting to has been written in Phoenix, which I am very familiar with, so I can easily submit PRs with suggestions.
