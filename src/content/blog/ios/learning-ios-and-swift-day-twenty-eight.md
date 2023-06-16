---
title: "Learning iOS and Swift. Day 28: Signing in over API"
date: 2022-06-15
slug: learning-ios-and-swift-day-twenty-eight
lang: en-US

summary: |
  Today I removed the mock from the login view and made the form actually call a login mutation upon submission.

---

Expanding on yesterday's code, I implemented the login mutation and integrated it with the login screen.
The application now calls the API, and upon success, it stores the access token and a refresh token on the keychain, and lets the user into the application.
With a MVVM architecture, it seems like there is quite a lot of boilerplate required to pass closures around.
Tomorrow I will try to improve the application's styling and layout and try to fetch some data in the application view.
