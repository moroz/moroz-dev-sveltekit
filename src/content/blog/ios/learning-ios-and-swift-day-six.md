---
title: "Learning iOS and Swift. Day 6: Structs and classes"
date: 2022-05-24
slug: learning-ios-and-swift-day-six
lang: en-US

summary: "Briefly discussing structs and classes, with key differences, property and method declaration, and property observers."
---

There are three types of "algebraic" data types in Swift: enumerations, structs, and classes.
That's one more than Rust, which does not have classes.

## Classes vs. Structs

In Swift, structs and classes are very similar in functionality and behavior.
The key differences I have noticed by now is that classes can be subclassed (i. e. they support inheritance), and the way they are passed around in memory: structs are passed by value (like `Copy` values in Rust), while class instances are passed by reference (so, passing pointers around).

This can be easily illustrated by implementing a simple counter. With a struct, it looks like this:

```swift
struct StructCounter {
    private(set) var value: Int = 0

    mutating func inc() {
        self.value += 1
    }

    mutating func dec() {
        self.value -= 1
    }
}
```

Notice that within a struct, methods that modify the value of the struct's properties must be annotated with `mutating`.

```swift
var s1 = StructCounter()
s1.inc() // s1.value is now 1

var s2 = s1 // s1 gets copied
s2.inc()

print(s1.value) // 1
print(s2.value) // 2
```

By annotating the `value` property with `private(set)`, we prevent the struct from being modified from the outside of the struct. If we tried to reassign the value nevertheless, we would get a compiler error:

```swift
myCounter.value = 100
```

```shell
struct_with_methods.swift:21:11: error: cannot assign to property: 'value' setter is inaccessible
myCounter.value = 100
~~~~~~~~~~^~~~~
```

The same counter functionality, but written as a class:

```swift
class ClassCounter {
    private(set) var value: Int = 0

    func inc() {
        self.value += 1
    }

    func dec() {
        self.value -= 1
    }
}
```

In class definitions, mutating methods must not be annotated with `mutating`.

```swift
var c1 = ClassCounter()
c1.inc() // c1.value is now 1

var c2 = c1 // c2 and c1 now point to the same instance
c2.inc() // so when you call a method on c2, c1 gets mutated as well

print(c1.value) // 2
print(c2.value) // 2
```

## Property observers

It is possible to execute some code after (or before) a property has been set on a struct or class instance.
This is done using a feature called "property observers," which are a nice alternative to setter methods.
An example use case of property observers is in an Active Record-like data structure, representing a record in the database.
Before validating and writing string input, we need to trim all surrounding whitespace from a string.
This section is inspired by an example from the book <em>[Swift in depth](https://www.manning.com/books/swift-in-depth)</em> by Tjeerd in 't Veen.

```swift
import Foundation

struct User {
    let id: String
    var name: String {
        didSet {
            self.name = self.name.trimmingCharacters(in: .whitespaces)
        }
    }

    init(name: String) {
        defer { self.name = name }
        self.id = UUID.init().uuidString.lowercased()
        self.name = ""
    }
}
```

This example defines a struct called `User` with two properties: an immutable `id` set in the initializer, and a mutable `name`.
I also defined a `didSet` closure that trims the `name` after it has been set:

```swift
var name: String {
    didSet {
        self.name = self.name.trimmingCharacters(in: .whitespaces)
    }
}
```

The way to trim whitespaces from a string in Swift is using:

```swift
string.trimmingCharacters(in: .whitespaces)
```

Note that this method is provided by Foundation. Another feature from the Foundation framework is the UUID struct, providing methods to generate and manipulate UUIDs.

In the initializer, I generate a random UUID to use as the record's unique identifier.
In this example, I treat the UUID as a string, because the actual data representation of a UUID is not relevant to this example.
I initialize `name` as an empty string to let the compiler know that the value will always be initialized in the initializer.
The actual initial value for `name` is set in a `defer` closure, which will be fired after the rest of the initialization code.
This is because the `didSet` closure does not fire in the initializer.

```swift
init(name: String) {
    defer { self.name = name }
    self.id = UUID.init().uuidString.lowercased()
    self.name = ""
}
```

Let's try this out:

```swift
var newUser = User(name: "   Test   ")
print(newUser)
// User(id: "ba3d5e23-431c-404c-a371-8ed306f07900", name: "Test")
```

As expected, upon initialization, all surrounding whitespace in the `name` is trimmed.
What happens when we set a new value?

```swift
newUser.name = "   A changed value "
print(newUser)
// User(id: "ba3d5e23-431c-404c-a371-8ed306f07900", name: "A changed value")
```

Here, too, the value has been correctly trimmed.

Besides `didSet`, you can also add a `willSet` observer, but it seems like it cannot be used to manipulate the property value:

```swift
var name: String {
    didSet {
        self.name = self.name.trimmingCharacters(in: .whitespaces)
    }
    willSet {
        print("About to change the value to \(newValue)")
    }
}
```
