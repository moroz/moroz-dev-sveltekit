---
title: "Learning iOS and Swift. Day 21: Project update, Decodable protocol"
date: 2022-06-08
slug: learning-ios-and-swift-day-twenty-one
lang: en-US

summary: |
  Today I implemented persisting practice history each time the user adds some repetitions of a practice.
  I also implemented a simple view for presenting those records.
  I found out how to implement custom keys for `Decodable` data structures.

---

## Upserting practice history records

Today I have been working on improving my side project, _Ngöndro Tracker_ (more details in [day 19 post](/blog/learning-ios-and-swift-day-nineteen)).
I finally implemented storing a practice history, so now each time the user adds several rounds of mantras on a single day, the total amount will be stored in a single database entry for that day. If you think this sounds like a perfect use case for an upsert, you're right.
Below is the function that stores the information in the database:

```swift
static func addAmount(store: DataStore, practiceId: Int, amount: Int) throws {
  let query = """
    insert into practice_sessions (practice_id, amount, date) values (?, ?, ?)
    on conflict (practice_id, date)
    do update set amount = amount + excluded.amount;
    """

  try store.db.run(query, practiceId, amount, Date().dateString)
}
```

SQLite, being limited in terms of data types and advanced query features, it still does support compound unique indices and `on conflict` clauses, which is a blessing for someone coming from a backend development background, where Postgres is the king of all data stores and every day one has to interface with SQL databases.
In the example above, `Date().dateString` is a computed property I monkey-patched on top of `Date` to return a string with just the date:

```swift
import Foundation

extension Date {
  var dateString: String {
    let formatter = DateFormatter()
    formatter.dateFormat = "YYYY-MM-dd"
    return formatter.string(from: self)
  }
}
```

## Custom field names in Decodable protocol

I found out how to leverage [SQLite.swift](https://github.com/stephencelis/SQLite.swift)'s support for encoders and decoders while conforming to SQL's and Swift's naming conventions by implementing a `CodingKeys` enum on the `Codable` structure:

```swift
import Foundation
import SQLite

struct PracticeSession: Identifiable, Codable {
  // ...

  var id: Int
  var practiceId: Int
  var date: String
  var amount: Int
  
  enum CodingKeys: String, CodingKey {
    case id
    case practiceId = "practice_id"
    case date
    case amount
  }

  // ...

  static func filterByPractice(store: DataStore, practiceId pid: Int) throws -> [Self] {
    let query = sessions.filter(practiceId == pid).order(date.desc)

    do {
      let rows: [Self] = try store.db.prepare(query).map { row in
        return try row.decode()
      }
      return rows
    } catch {
      throw DatabaseError.queryError
    }
  }
}
```

With this implementation, the struct may use camel-cased properties while keeping snake-cased column names in the database.

## History view

Armed with a data structure and a method to save some data to it, I implemented a `HistoryView` and added a `NavigationLink` to that view to the `PracticeView`.

<figure class="dark-light-figure">
  <picture>
    <source srcset="/images/ios-21/practice-view.webp" media="(prefers-color-scheme: light)" />
    <source srcset="/images/ios-21/practice-view-dark.webp" media="(prefers-color-scheme: dark)" />
    <img src="/images/ios-21/practice-view.webp" />
  </picture>
  <figcaption>Practice view with the new navigation link to the history view.</figcaption>
</figure>

In the history view, I fetch practice history data from the data store and present it in a `List`.

<figure class="dark-light-figure">
  <picture>
    <source srcset="/images/ios-21/history-view.webp" media="(prefers-color-scheme: light)" />
    <source srcset="/images/ios-21/history-view-dark.webp" media="(prefers-color-scheme: dark)" />
    <img src="/images/ios-21/history-view.webp" />
  </picture>
  <figcaption>History view, presenting practice session days in a list.</figcaption>
</figure>

```swift
import SwiftUI

struct HistoryView: View {
  let practice: Practice

  @EnvironmentObject var dataStore: DataStore
  @State private var entries: [PracticeSession] = []

  func loadEntries() throws {
    do {
      entries = try PracticeSession.filterByPractice(store: dataStore, practiceId: practice.id)
    } catch {
      print(error)
    }
  }

  var body: some View {
    List(entries) { entry in
      HStack {
        Text(String(entry.amount))
        Spacer()
        Text(entry.date)
      }
    }
    .navigationTitle(Text(practice.name))
    .onAppear { try? loadEntries() }
  }
}
```
