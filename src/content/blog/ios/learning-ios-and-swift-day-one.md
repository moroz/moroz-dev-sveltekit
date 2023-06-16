---
title: "Learning iOS and Swift. Day 1: Strings"
date: 2022-05-20
slug: learning-ios-and-swift-day-one
lang: en-US

summary: |
  Learning notes from the first day learning iOS and Swift development.
  Reversing, uppercasing, interpolating strings in Swift.
---

## Mission statement

Learn iOS development for 30 minutes a day, for 30 days.

## Rationale

Rationale: I have been procrastinating learning new things for too long, telling myself that I will learn this or that new language once I finish this or that client project. And when I finish some project, there are new client projects to work on and the whole vicious cycle repeats. This time, I decided to go for Swift and iOS, because I have wanted to build mobile apps for a long time (and React Native is not good enough).

### What I learned on day 1

`let` defines immutable variables while `var` defines mutable variables.

```swift
// var defines a mutable variable
// String type is called String
var greeting = "Hello, playground"

// let defines an immutable variable
// Defaults to Int type (i32?)
let x = 10
```

Floating point numbers are inferred to be `Double` type by default.

```swift
let double = 21.37
```

Uppercasing and lowercasing strings is simple, using `.lowercased()` and `.uppercased()` methods on the string itself:

```swift
let uppercaseGreeting = greeting.uppercased()
let lowercaseGreeting = greeting.lowercased()
```

Reversing a string is more complicated as the `.reversed()` method returns a vector of Unicode graphemes rather than a string, so we need to coerce it back to `String`:

```swift
let reversed = String("deified".reversed())
```

Now that we know how to reverse strings, we can write a function that checks if a given string is a palindrome:

```swift
// prefixing argument with _ to make it positional
// type annotations for arguments and return types are required
func isPalindrome(_ str: String) -> Bool {
    let reversed = String(str.reversed())
    return reversed == str
}

isPalindrome("deified") // true
isPalindrome("abacad") // false
```

String interpolation syntax: `\( expression )` (backslash followed by the expression in brackets).

```swift
// string interpolation

let amount = 21.37
// How much wood would a woodchuck chuck if a woodchuck could chuck wood?
let msg = "Roughly \(amount) metric tonnes."
```
