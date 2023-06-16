---
title: "Learning iOS and Swift. Day 12: Observable objects, environment objects"
date: 2022-05-30
slug: learning-ios-and-swift-day-twelve
lang: en-US

summary: |
  Describing observable objects, which act like event-driven data sources, and `@EnvironmentObject`s,
  which expose a given object to the entire view tree.

---

Following chapter 7 of the [SwiftUI Apprentice](https://www.raywenderlich.com/books/swiftui-apprentice) book, I have implemented a timer component that counts seconds down to 0, and a global data store of exercises completed by the user.

## The timer

Below is the full implementation of the timer view:

```swift
import SwiftUI

struct TimerView: View {
  @State private var timeRemaining = 3
  @Binding var timerDone: Bool

  let timer = Timer.publish(every: 1, on: .main, in: .common)
    .autoconnect()

  var body: some View {
    Text("\(timeRemaining)")
      .font(.system(size: 90, design: .rounded))
      .padding()
      .onReceive(timer) { _ in
        if self.timeRemaining > 0 {
          self.timeRemaining -= 1
        } else {
          timerDone = true
        }
      }
  }
}
```

The timer used by the timer view is implemented using a class from the Foundation library called [Timer.TimerPublisher](https://developer.apple.com/documentation/foundation/timer/timerpublisher).
As we can see, the timer itself is stored as an immutable property on the view, and is instantiated using `Timer.publish(every:on:in:)`.
The arguments passed to the initializer define the time interval at which events are published (every 1 second), the _run loop_ on which they run (`.main`), and a [mode](https://developer.apple.com/documentation/foundation/runloop/mode/) for that run loop (`.common`).
The timer starts publishing events only after a subscriber is connected, which in this case is done using the `.autoconnect()` method.
The actual events are consumed in the renderer, and are handled using the `.onReceive()` method on the main view.
Apparently, state and bindings can only be mutated explicitly in the renderer function, so there is no easy way to refactor all those closures into instance methods.

## The global data store

Below is the implementation of the `HistoryStore` class that is later used as a global data store:

```swift
import Foundation

struct ExerciseDay: Identifiable {
  let id = UUID()
  let date: Date
  var exercises: [String] = []
}

class HistoryStore: ObservableObject {
  @Published var exerciseDays: [ExerciseDay] = []

  init() {
    #if DEBUG
      seedData()
    #endif
  }

  func addDoneExercise(_ exerciseName: String) {
    let today = Date()
    if today.isSameDay(as: exerciseDays[0].date) {
      print("Adding \(exerciseName)")
      exerciseDays[0].exercises.append(exerciseName)
    } else {
      exerciseDays.insert(
        ExerciseDay(date: today, exercises: [exerciseName]), at: 0)
    }
  }
}
```

The store must be a class rather than struct, and must implement the `ObservableObject` protocol.
I am not entirely sure what effect the `@Published` modifier has, but I assume it makes the `exerciseDays` property reactive.
There is a method that adds a given exercise to the exercise history, with a relatively straightforward implementation.
Note that the `today.isSameDay(as:)` call on a `Date` struct is not part of the standard library, but is an extension of the `Date` type provided by the authors of the book. The implementation of that method is as follows:

```swift
extension Date {
  /// Check another date is the same year, month and day.
  ///   - parameters:
  ///     - day: The other date.
  func isSameDay(as day: Date) -> Bool {
    yearMonthDay == day.yearMonthDay
  }
}
```

The history store is later defined as an _environment object_, which behaves more or less like a React context, exposing the object to all components in the component tree, regardless of their nesting level.
However, unlike React contexts, which can have _any_ value, including `undefined`, an environment object must be an instance of a class implementing the `ObservableObject` protocol.

The object is exposed to child views like so:

```swift
struct ContentView: View {
  @State private var selectedTab = 9

  var body: some View {
    TabView(selection: $selectedTab) {
      WelcomeView(selectedTab: $selectedTab)
        .tag(9)
      ForEach(0..<Exercise.exercises.count) { index in
        ExerciseView(index: index, selectedTab: $selectedTab)
          .tag(index)
      }
    }
    .environmentObject(HistoryStore()) // <-- here
    .tabViewStyle(PageTabViewStyle(indexDisplayMode: .never))
  }
}
```

The `.environmentObject()` method is called on a view, with the value to be passed as its parameter.
In child views, the value of the environment object can be accessed using a struct property with the `@EnvironmentObject` attribute:

```swift
@EnvironmentObject var history: HistoryStore
```

At this point, I am not sure how the APIs decide what objects should be passed as which environment object (by type?), and whether it is possible to use multiple objects at once.

Thank you for reading, and I will see you in the next post.
