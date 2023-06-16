---
title: "Learning iOS as a Web Developer: One Month Later"
date: 2022-06-19
slug: learning-ios-as-a-web-developer
draft: true

---

A month ago, I set myself a goal to study iOS development with Swift for at least 30 minutes a day, every single day, without breaks and excuses.
For most of that time, I documented my progress on this blog, trying to at least outline what I studied on that particular day.
I haven't written any prose these past two days, because my efforts were concentrated mostly on writing Swift code.
Apparently, writing more code than prose has been working out better for study purposes.

## Swift is more fun than I thought

Swift comes with three categories of complex types: classes, structures and enumerations.
You can define methods and computed properties on all three, and you can extend existing types, including ones that you don't own, like `Date` or `String`.
Though everything is statically typed, satisfying type constraints is relatively easy using generics and protocols.
You can even define methods and computed properties on protocols, like a `.prettyPrinted` computed property on the `Codable` protocol.

For most expressions, the type of the expression can be inferred and does not need to be annotated, but in places where type annotations are required, like in function heads, Xcode can generate a huge part of the code for you.

## React Native is bulky and slow

## Xcode is getting better

For a long period of time, I have dreaded Xcode, for several reasons.
Firstly, because it's a 13 GB download and takes ages to install on M1 machines.
I already have a text editor of choice and was not happy to be torn away from the warmth of the terminal session.
I have spent so many years working in simple text editor that I don't feel the need for an IDE any longer.

In most IDEs, there is some way to emulate Vim.
In Emacs, the emulation goes as far as to implement the whole functionality of Vim, including macros and Ex commands such as `:%s` or `:sort`.
For some use cases, the Vim emulator works even better than Vim itself.
The last time I tried to learn Swift, setting up Vim keybindings required downloading a plugin, building and signing it.
These days, a limited set of Vim keybindings is included in Xcode out of the box.
However, for serious editing I would still use Vim, hands down.

## Mobile development is very different from the Web

On the Web, forms have worked exactly the same for the past 20 years or so: you wrap a few `<input>`s in a `<form>`, add a `<button type="submit">` somewhere, and the browser would handle serializing the form for you.
In native applications, there is no such thing as a `<form>`.
You need to handle input from all fields on your own and handle submission 

## Encoding and decoding data in a static language

`Codable` protocol

## Persistence

SQLite, Core Data
