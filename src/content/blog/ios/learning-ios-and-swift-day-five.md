---
title: "Learning iOS and Swift. Day 5: Enums"
date: 2022-05-23
slug: learning-ios-and-swift-day-five
lang: en-US

summary: "Quick introduction to things you can do with Swift enumerations."
---

Swift comes with a feature-rich system of data types, including enumerations, structs and classes.
All of these are quite similar to their Rust counterparts, except that in Rust, there are no classes.
Today I introduce some facts about enumerations, also known as `enum`s.

In order to be able to quickly test Swift scripts, I defined a keybinding to run the script inside Neovim:

```vim
nnoremap <Leader>mr :vert ter swift %<CR>a
```

A Swift enumeration, in its simplest form, is defined with just a name and cases.
Its values are abstract values with no `rawValue` property:

```swift
enum Status {
    case success, failure, unknown
}

print(Status.success)
// success
```

You can get a string representation of the enum label using `String(describing:)`:

```swift
let asString = String(describing: Status.unknown)
print(asString.uppercased())
// UNKNOWN
```

You can also define an enum to correspond to a raw value, like a string or a number.
If you just define an enum with an `Int` type, it will assign consecutive integer values, starting from 0:

```swift
enum Implicit: Int {
    case first
    case second
}

print(Implicit.first.rawValue)
// 0
print(Implicit.second.rawValue)
// 1
```

You can also define a staring value for one of the values, and the following cases will increment, starting from that value:

```swift
enum StatusCode: Int {
    case OK = 200
    case Created, Accepted, NonAuthoritativeInformation, NoContent
}

print(StatusCode.Created.rawValue)
// 201
print(StatusCode.NoContent.rawValue)
// 204
```

With String values, you need to define each value:

```swift
enum Role: String {
    case regular = "REGULAR"
    case admin = "ADMIN"
}

let adminRole = Role.admin
print(adminRole.rawValue)
// ADMIN
print(String(describing: adminRole))
// admin
```

## Converting from raw value to enum

You can convert a raw value to enum using an initializer with `rawValue` (`init(rawValue:)`).
The initializer returns an `Optional`, since the operation may fail:

```swift

// casting String to enum
if let fromString = Role(rawValue: "ADMIN") {
    print(fromString)
    // admin
}

let willFail = Role(rawValue: "NO CAN DO")
print(willFail as Any)
// nil
```

## Methods and computed properties

Enums, just like classes and structs, can have methods and computed properties.

```swift
// Foundation extends String with the `capitalized`
// computed property
import Foundation

// defining an enum with corresponding String values
enum FellowshipMember: String {
    case gandalf = "You shall not pass!"
    case frodo = "Okay, then. Keep your secrets."

    // a method
    func showYourself() {
        let label = String(describing: self).capitalized
        print("\(label) said, \"\(self.rawValue)\"")
    }

    // a computed property needs to be defined with `var`
    var spongeBobCased: String {
        let converted = self.rawValue.unicodeScalars.enumerated().map { (index, char) -> String in
            let str = String(char)
            if index % 2 == 0 {
                return str.lowercased()
            } else {
                return str.uppercased()
            }
        }
        return converted.joined()
    }
}

print(FellowshipMember.gandalf.spongeBobCased)
// yOu sHaLl nOt pAsS!
print(FellowshipMember.frodo.spongeBobCased)
// oKaY, tHeN. kEeP YoUr sEcReTs.

FellowshipMember.frodo.showYourself()
// Frodo said, "Okay, then. Keep your secrets."
FellowshipMember.gandalf.showYourself()
// Gandalf said, "You shall not pass!"
```
