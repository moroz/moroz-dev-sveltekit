---
title: "Learning iOS and Swift. Day 15: Optionals are syntax sugar for Rust-like enums; presenting case study"
date: 2022-06-02
slug: learning-ios-and-swift-day-fifteen
lang: en-US

summary: |
  Brief progress update on issues from the book.
  I present the mobile app I want to work on henceforth.

---

## Optionals _are_ Rust-like enums under the hood

Swift's syntax made me believe that optionals are implemented as some special form rather than a two-case enum like `Option<T>` in Rust.
I was wrong. Swift's optionals are implemented in exactly the same way, and the commonly used syntax (literal value for `.some(Wrapped)` or `nil` for `.none`) is just syntax sugar.
To test it, I wrote this snippet using Rust-like optional syntax:

```swift
// I typed this as Array<Optional<String>>,
// but swift-format converted it to this shorthand
let array: [String?] = [.some("Hello world!"), .none]

for elem in array {
  switch elem {
  case let .some(str):
    print("String found: \(str)")
  case .none:
    print("No string here!")
  }
}
```

And, lo and behold, it works exactly as expected:

```shell
$ swift optionals.swift
String found: Hello world!
No string here!
```

## Progress update

Today I followed chapter 9 of [SwiftUI Apprentice](https://www.raywenderlich.com/books/swiftui-apprentice).
The chapter dealt with persistent storage, saving current application data to property lists, and restoring data from property lists to state on boot.
I also learned how to display alerts in SwiftUI, and even that is declarative.

## TIL about exception handling

* exceptions are enums
* functions that may throw must be marked with `throw`
* `try?` works like `.ok()` in Rust, i. e. converts a result to an optional
* `as?` casts a value to another type; the result is optional

## Case study

As my study project, I intend to develop an application which will basically recreate [iMala - meditation tracker](https://apps.apple.com/us/app/imala-meditation-tracker/id1249800614?platform=iphone).
This application is used to track a user's progress in their Four Preliminary Practices, which are a set of four meditation practices used by practicioners of Diamond Way Buddhism.
However, the app currently only supports iPhone and iPad, with no integration with Apple Watch.
I could contact the developer and ask him for the source code, but the app is so old that it's definitely written in UIKit, and possibly even in Objective-C.
I will be working on this research project and the book about 50/50.
