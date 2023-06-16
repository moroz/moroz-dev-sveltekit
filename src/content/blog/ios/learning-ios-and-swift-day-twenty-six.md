---
title: "Learning iOS and Swift. Day 26: Keychain storage, MVVM architecture"
date: 2022-06-13
slug: learning-ios-and-swift-day-twenty-six
lang: en-US

summary: |
  Exploring integration with Apollo Client for iOS. Discovering MVVM architecture and singleton classes.
  Storing authentication data inside the Keychain using the Security module, which is part of Foundation.

---

Yesterday I more or less finished Apple's SwiftUI tutorial, skipping the section on macOS development, as it is outside my area of focus at the moment.
I tried to build a watchOS app for my Ngöndro Tracker project, but I hit a wall due to persistence and synchronization requirements.
As it turned out, even though SQLite3 is readily available on iOS, iPadOS, and macOS, it is not available on watchOS, probably because it is too resource-consuming.
Having overlooked this fact, I could only watch each build of the watchOS target fail miserably, not finding this or that header file.
I am planning to approach this project again once I learn enough about Core Data to be able to use it for persistence both on the iPhone as well as on the Apple Watch.

## Storing credentials on the keychain

Over the course of the following weeks, I am planning to learn enough about Core Data and Apollo Client integration to be able to build a mobile app for my friends' side project, [GouGou.cash](https://gougou.cash/).
GouGou is an expense tracker app, currently built as a React PWA with a GraphQL backend, implemented in Absinthe for Elixir.
The first thing I tried today was connecting to the application's GraphQL API.
Before I could talk to an API, though, I figured I would need a way to store the user's API credentials, if present.
These credentials would be in the form of two JSON Web Tokens (JWTs): an access token and a refresh token.
As far as I know, in the browser, the best way to store those tokens would be in HTTP-only cookies, but when using an HTTP client inside of a native applications, requests probably don't store cookies like they would in a browser.

There is a feature built into all Apple operating system called "the keychain," and it is an encrypted storage for relatively small amounts of data, like API tokens, passwords, on key pairs.
Keychain APIs are part of the Foundation framework, and as such, they are most likely written in Objective-C and have rather clunky APIs.
For instance, to store a key on the keychain, you need to pass a `CFDictionary`, which is not a Swift dictionary, to the function `SecItemAdd(_:_:)`, which is not a constructor.
The keys in the dictionary are not `String`s, they are `CFString`s, and in general, there are many constants in the library that need to be coerced to and from `String`s in order to satisfy Swift's type constraints.

For this project, I implemented a helper class based on the article [Persisting Sensitive Data Using Keychain in Swift](https://swiftsenpai.com/development/persist-data-using-keychain/) by Lee Kah Seng, the Swift Senpai.
I will not include the whole class in this article, but I will just mention a quirk of the Security framework which prevented the code from the article from compiling.

Below is a fragment of working code that stores a credential on the keychain:

```swift
let query: [String: Any] =
  [
    kSecValueData as String: data,
    kSecClass as String: kSecClassGenericPassword,
    kSecAttrService as String: service,
    kSecAttrAccount as String: account,
  ]

let status = SecItemAdd(query as CFDictionary, nil)
```

As you can see, even though the `query` dictionary is annotated as `[String: Any]`, you still need to explicitly cast each `CFString` constant as `String`, and then all of that needs to be explicitly cast as `CFDictionary`.
The `status` as returned from the function is an `OSStatus`, which is a wrapper for a very long list of negative integers, which in turn is a common practice in C code.

## Model-View-ViewModel pattern

I noticed that some tutorials and learning materials advocate a design pattern called MVVM (Model-View-ViewModel), which seems to nicely separate presentation logic from such concerns as event handling and data fetching.
It is the same pattern as the one used by Vue.js.
I will try to learn this pattern and use it in my future code.
