---
title: "Learning iOS and Swift. Day 3: Similarities to Rust"
date: 2022-05-21
slug: learning-ios-and-swift-day-three
lang: en-US

summary: |
  Looking at some of Swift's abstract types, such as enums,
  I noticed how similar they are to Rust.
  In this post, I also describe common operations on date and time values.

---

## Date and time operations

Today I explored common operations related to date and time values.
In order to use the `Date` struct and its associated functions, we need
to import the Foundation framework, which is a part of the Swift standard
library.

```swift
import Foundation
```

Originating from early NeXTStep days, Foundation carries a lot
of legacy of the early Apple operating systems. Most notably, the library
includes many data types and classes prefixed with `NS`, such as `NSDate`,
the `NS` standing for NeXTStep.

The simplest thing we may want to do with a date is to instantiate a new
`Date` with the current date and time:

```swift
// current time in UTC
let now = Date()
```

If you need an epoch time (Unix timestamp), the struct has a property for that.
The property is of type `TimeInterval`, but it can be cast as `Int`:

```swift
let unix = Int(now.timeIntervalSince1970)
```

To do the opposite, i. e. to get a `Date` from a Unix timestamp:

```swift
// TimeInterval seems to be an alias for Double
let someUnixTimestamp: TimeInterval = 1401667200

// Date from Unix time
let dateFromUnix = Date(timeIntervalSince1970: someUnixTimestamp)

print("Swift was first announced on \(dateFromUnix)")
// Swift was first announced on 2014-06-02 00:00:00 +0000
```

To add a time interval to the current timestamp, (e. g. calculate
an hour from now):

```swift
// an hour from now
let hourFromNow = Date(timeIntervalSinceNow: 3600)
```

There is no separate initializer for doing the reverse, i. e. calculating
an hour ago, but you can pass a negative value to achieve the same result:

```swift
// an hour ago
let hourAgo = Date(timeIntervalSinceNow:  -3600)
```

To format a timestamp as ISO 8601 time (the format used in JSON serialization):

```swift
// format a Date as ISO 8601
let nowAsISOTime = ISO8601DateFormatter().string(from: now)
```

To parse an ISO string as timestamp:

```swift
// parsing an ISO 8601 string as date
let isoString = "2010-04-10T21:37:00Z"

// Wrapping this expression in optional binding since it is an Optional
if let parsed = ISO8601DateFormatter().date(from: isoString) {
    print(parsed)
}
```

## Optionals and optional bindings

Note the use of optional bindings in the last snippet. The syntax is almost identical to Rust's, (in fact, the syntax in Rust has been [borrowed from Swift](https://doc.rust-lang.org/reference/influences.html)), except that Rust's optionals are enums of type `Option<T>` and when pattern matching, you need to match with `if let Some(value) = ...`:

```rust
let my_vec: Vec<Option<i32>> = vec![Some(2137), None];

for value in my_vec {
    // Here I'm also shadowing the variable `value`
    if let Some(value) = value {
        println!("Encountered an Int: {}", value);
    } else {
        println!("No value here");
    }
}
```

```shell
$ rustc optional.rs                   1
$ ./optional
Encountered an Int: 2137
No value here
```

The equivalent in Swift would be:

```swift
let my_vec: [Int?] = [2137, nil]

for value in my_vec {
    // Shadowing local variables is not permitted in Swift
    if let val = value {
        print("Encountered an Int: \(val)")
    } else {
        print("No value here")
    }
}
```

```shell
$ swift optional.swift
Encountered an Int: 2137
No value here
```

In Rust, there is an `Option<T>` type for nullable values and a `Result<T, E>`
type for the results of operations that may fail. The Optional types in Swift
seem to be used for both.

## Formatting dates and times

Another extremely common use case is formatting dates and times as human-readable strings (like the date and time written on the lock screen of your iPhone). Foundation comes with a `DateFormatter` struct that does this and is highly configurable.

```swift
let dateFormatter = DateFormatter()
```

`DateFormatter` instances have the properties `dateStyle` and `timeStyle`, both of which default to `DateFormatter.Style.none`.
Before you can format any string, you need to set a format for either, otherwise you will end up with an empty string:
  
```swift
print(dateFormatter.string(from: now))
// returns empty string
```

The possible values of [`DateFormatter.Style`](https://developer.apple.com/documentation/foundation/dateformatter/style)
are `none`, `short`, `medium`, `long`, and `full`. If you want only the date or the time part, set either property to `.none`.

```swift
dateFormatter.dateStyle = DateFormatter.Style.medium
// since the properties are statically typed, we can omit the enum name
dateFormatter.timeStyle = .medium
// May 21, 2022, 6:03:31 PM
```

```swift
dateFormatter.dateStyle = .long
dateFormatter.timeStyle = .long
// May 21, 2022 at 6:03:31 PM GMT+2
```

```swift
dateFormatter.dateStyle = .full
dateFormatter.timeStyle = .full
// Saturday, May 21, 2022 at 6:03:31 PM Central European Summer Time
```

The formatters support a wide range of locales.

```swift
// Set a custom locale
dateFormatter.locale = Locale(identifier: "zh_TW")
print(dateFormatter.string(from: now))
// 2022年5月21日 星期六 下午6:09:51 [中歐夏令時間]

dateFormatter.locale = Locale(identifier: "zh_CN")
print(dateFormatter.string(from: now))
// 2022年5月21日星期六 中欧夏令时间 下午6:09:51

dateFormatter.locale = Locale(identifier: "pl_PL")
print(dateFormatter.string(from: now))
// sobota, 21 maja 2022 18:09:51 czas środkowoeuropejski letni
```
