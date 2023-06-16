---
title: "Learning iOS and Swift. Day 23: Project update: saving data in settings view"
date: 2022-06-10
slug: learning-ios-and-swift-day-twenty-three
lang: en-US

summary: |
  I implemented saving data in the practice edit view.
  I found out how to programmatically return from a `NavigationLink`.

---

Yesterday I built a settings view that allows the user to set certain settings for each practice, such as title, current amount (because often users already have some repetitions accumulated), mala size, and target amount.
Today I had to write a function that saves the data to the database and returns to the practice view, hopefully with updated data.
In the edit view, all edits are performed on a copy of the record, which is then saved when the user clicks "Save."
Since the current amount is cached in the state of the parent view, this value had to be passed in as binding, so that it could be correctly updated after the edit.
Finally, after saving, I reload all the practices in the data store, because it's the easiest way to bust cache.

The view is rendered through a `NavigationLink`.
As it turns out, the way to return _from_ a navigation link is the same as with modal sheets, using `@Environment(\.presentationMode)`.
The presentation mode is passed to the view as some sort of special value, which can be unwrapped, and then you can call `.dismiss()`.

```swift
import SwiftUI

struct EditPractice: View {
  @EnvironmentObject var dataStore: DataStore
  @Environment(\.presentationMode) var presentationMode
  var practice: Practice
  @Binding var amount: Int
  @State var draft: Practice = Practice()

  func saveChanges() {
    if let _ = try? draft.save(store: dataStore) {
      amount = draft.currentAmount
      try? dataStore.loadPractices()
    }
    presentationMode.wrappedValue.dismiss()
  }

  var body: some View {
    ScrollView(.vertical) {
      VStack(alignment: .leading) {
        LabeledTextField(label: "Name", value: $draft.name)
        LabeledTextField(label: "Mala size", value: $draft.malaSize)
        LabeledTextField(label: "Current amount", value: $draft.currentAmount)
        LabeledTextField(label: "Target amount", value: $draft.targetAmount)
      }
    }
    .navigationTitle("Edit practice")
    .toolbar {
      Button("Save") { saveChanges() }
    }
    .onAppear {
      draft = practice
    }
  }
}
```
