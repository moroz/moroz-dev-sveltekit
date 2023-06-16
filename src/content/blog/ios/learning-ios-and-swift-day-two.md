---
title: "Learning iOS and Swift. Day 2: Setting up environment and Vim"
date: 2022-05-20
slug: learning-ios-and-swift-day-two
lang: en-US

summary: |
  Setting up Neovim and Swift toolchain on Debian Linux.
  Writing a CLI program in Swift.
---

Today I decided that before I get my hands dirty with iOS development, I want to gain a thorough understanding of the Swift programming language.
Swift's syntax is oftentimes not too intuitive and when learning a new technology, I don't like to be distracted by language constructs or idioms that I don't fully understand.

This means that before writing iOS apps, I can stay away from Xcode and learn the language itself on Linux.

## Installing Swift toolchain on Linux

My Linux laptop runs Debian 11, so I downloaded the toolchain for Ubuntu 20.04 (amd64) from [Swift's website](https://www.swift.org/download/).

I installed its dependencies using a snippet copied from the Website:

```shell
$ sudo apt-get install binutils git gnupg2 libc6-dev libcurl4 libedit2 libgcc-9-dev \
  libpython2.7 libsqlite3-0 libstdc++-9-dev libxml2 libz3-dev pkg-config tzdata \
  uuid-dev zlib1g-dev
```

I downloaded a tarball with the toolkit and extracted it to a folder (I opted for `~/ios`). I then added `~/ios/usr/bin` to the `PATH` in `~/.zshrc`.

## Syntax highlighting and intellisense in Neovim

My everyday programming IDE and text editor is [Neovim](https://neovim.io/) with [Conquer of Completion](https://github.com/neoclide/coc.nvim) for language server (LSP) support.

The Swift toolkits (both the one bundled with Xcode and the Linux tarball) come with an LSP server called `sourcekit-lsp`. It is compatible with a CoC plugin called [coc-sourcekit](https://github.com/klaaspieter/coc-sourcekit).
To get it working on Linux, I needed to set the `sourcekit.commandPath` property in `~/.config/nvim/coc-settings.json`:

```json
{
  "sourcekit.commandPath": "~/ios/usr/bin/sourcekit-lsp"
}
```

## Building a simple CLI in Swift

I took the function to check for palindromic strings from day 1 and wrapped it in a CLI. This is the program in its entirety:

```swift
import Foundation

func isPalindrome(_ str: String) -> Bool {
    return String(str.reversed()) == str
}

if CommandLine.arguments.count == 1 {
    print("Usage: palindrome WORDS")
    exit(0)
}

for word in CommandLine.arguments.suffix(from: 1) {
    if isPalindrome(word) {
        print("\(word) is a palindrome!")
    } else {
        print("\(word) is not a palindrome.")
    }
}
```

Let me analyse the new parts.

```swift
import Foundation
```

I import Foundation (whatever it is) to bring into scope the function `exit` (terminating the process with a given code).

```swift
if CommandLine.arguments.count == 1 {
    print("Usage: palindrome WORDS")
    exit(0)
}
```

The CLI checks each word in ARGV to see if it is a palindrome. If no words were passed to the program, it prints a help message and exits with code 0.
Here, I am using `CommandLine.arguments.count` (although I could probably be using something like `CommandLine.argc`). Since the first element of ARGV is always the name of the program, a count of 1 means that no additional arguments were passed to the CLI.

```swift
for word in CommandLine.arguments.suffix(from: 1) {
    if isPalindrome(word) {
        print("\(word) is a palindrome!")
    } else {
        print("\(word) is not a palindrome.")
    }
}
```

This part iterates over all words in ARGV, starting from the second one.
The `.suffix(from: Int)` method slices a collection, starting from element with index `from` to the end of the collection.
There is also a `.prefix()` method that does the same, but on the other end of the collection.
Iteration is implemented using `for ... in ... { ... }`.

## The CLI in action

```shell
# swiftc is the compiler. You can also compile and run a Swift file
# using the swift command
$ swiftc palindrome.swift 
$ ./palindrome  
Usage: palindrome WORDS
$ ./palindrome girafarig deified abacaba bepis
girafarig is a palindrome!
deified is a palindrome!
abacaba is a palindrome!
bepis is not a palindrome.
```
