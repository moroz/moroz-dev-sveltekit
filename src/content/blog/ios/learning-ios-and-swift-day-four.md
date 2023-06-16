---
title: "Learning iOS and Swift. Day 4: A thing on dictionaries"
date: 2022-05-22
slug: learning-ios-and-swift-day-four
lang: en-US

summary: |
  Today I explore dictionaries, the Swift-flavored unique key-value collections.

---

Today I am going to write about dictionaries.
Dictionaries are collections of key-value pairs.
In dictionaries, a given key can only have one corresponding value.
In Elixir, there is another key-value collection called a "keyword list," which is basically just a list of key-value tuples.
In keyword lists, a given key can occur multiple times within the same collection, but all keys must be atoms.

Most programming languages seem to pride themselves in inventing new names for their key-value collections.
In JavaScript, this data type is called `object`, in Ruby--`Hash`, in Elixir and Erlang--`map`.
In Rust, it's a portmanteau: `HashMap`.
In PHP, it's an "associative array."
In Swift, similarly to Python's `dict`, it is called a "dictionary."

## Dictionary literals

In Swift, dictionary literals are delimited by square brackets (`[` and `]`) and key-value pairs are associated using colons (`:`).
Any value implementing the `Hashable` protocol (trait) can be a key. Common `Hashable`s include numbers and strings.

With integer keys:

```swift
let aDictionary = [
    1: "a",
    2: "b"
]

print(aDictionary)
// [2: "b", 1: "a"]
```

Note that the key-value pairs are printed out in a different order.
Unlike Ruby and JavaScript, which preserve the order in which their key-value pairs were declared, and unlike Elixir, whose maps are always sorted, Swift does not guarantee any deterministic order of keys in dictionaries.
Most of the time, this does not matter, because dictionaries are usually accessed by their keys.

With `Double` keys:

```swift
let doubleKeys = [
    21.37: "Pope John Paul II",
    420: "Blaze it"
]

print(doubleKeys)
// [21.37: "Pope John Paul II", 420.0: "Blaze it"]
```

Not that even though the number `420` has no decimal part, it is still inferred as a `Double`, `420.0`.

With `String` keys, remember to wrap all keys in double quotes:

```swift
let stringKeys = [
    "a": 1,
    "b": 2
]

print(stringKeys)
// ["a": 1, "b": 2]
```

If you omit the double quotes, the keys will be treated as expressions:

```swift
let someKey = "I am a string"
let otherKey = "I am another string"

let exprKeys = [
    someKey: 123,
    otherKey: 456
]

print(exprKeys)
// ["I am a string": 123, "I am another string": 456]
```

## Mixing key types

If you mix and match key types, their value cannot be automatically inferred:

```swift
let mixedKeys = [
    1: "a",
    "b": 2
]
```

```shell
$ swift dict/mixed_keys.swift
dict/mixed_keys.swift:1:17: error: heterogeneous collection literal could only be
inferred to '[AnyHashable : Any]'; add explicit type annotation if this is intentional
let mixedKeys = [
                ^
```

The solution to this problem is included in the compiler error: you need to annotate the mixed-key dictionary with the type `[AnyHashable: Any]`:

```swift
let mixedKeys: [AnyHashable: Any] = [
    "a": 1,
    2: "b"
]

print(mixedKeys)
// [AnyHashable(2): "b", AnyHashable("a"): 1]
```

## Empty dictionary literal

An empty dictionary literal is denoted as `[:]`. You need to explicitly annotate its type.

```swift
var empty: [String: Any] = [:]
print(empty)
// [:]
```

You can append values to the empty dictionary by assigning to its indices:

```swift
empty["An integer value"] = 123
empty["A double value"] = 21.37
empty["A string value"] = "Lo and behold"

print(empty)
// ["A double value": 21.37, "An integer value": 123, "A string value": "Lo and behold"]
```

## Enum keys

Simple enums (without associated) seem to be `Hashable` by default.

```swift
// define an enum for common currencies in my part of the world
enum Currency {
    case eur
    case pln
    case czk
}

let beerPriceIn2022: [Currency: Double] = [
    .czk: 35,
    .pln: 14,
    .eur: 5
]
```

To iterate on this dictionary, you can use a regular `for-in` clause:

```swift
for (currency, price) in beerPriceIn2022 {
    // convert enum case to String and convert it to upper case
    let printable = String(describing: currency).uppercased()
    print("Price of beer in 2022: \(price) \(printable)")
}
```

```shell
$ swift dict/enum_keys.swift
Price of beer in 2022: 45.0 CZK
Price of beer in 2022: 5.0 EUR
Price of beer in 2022: 14.0 PLN
```

## Serializing dictionaries to JSON

Serializing maps **into** JSON is fairly easy. The Foundation framework includes the class [`JSONEncoder`](https://developer.apple.com/documentation/foundation/jsonencoder).

```swift
import Foundation

let myDict = [
    "PL": "PLN",
    "CZ": "CZK",
    "SK": "EUR"
]

let encoder = JSONEncoder()

// enable pretty printing
encoder.outputFormatting = .prettyPrinted

let jsonData = try encoder.encode(myDict)
if let json = String(data: jsonData, encoding: .utf8) {
    print(json)
}
```
