---
title: "Learning iOS and Swift. Day 29: Customizing view for signed in user"
date: 2022-06-16
slug: learning-ios-and-swift-day-twenty-nine
lang: en-US
draft: true

summary: |
  I implemented presenting personalized messages based on the data for a signed in user.

---

Today I implemented passing an Authentication header along with GraphQL requests.
This is more involved than it sounds.
Unlike the Web, where the only thing you need to do to pass cookies along with your requests is pass `credentials: "include"`, on iOS you need some way to store the tokens and implement custom Apollo middleware (called "interceptor") to fetch those tokens if present.
The code initializing the client with a custom middleware stack looks like so:

```swift
import Apollo
import Foundation

class Network {
  static let shared = Network()

  private(set) lazy var apollo: ApolloClient = {
    let cache = InMemoryNormalizedCache()
    let store = ApolloStore(cache: cache)
    let url = URL(string: "http://localhost:4000/api")!
    let client = URLSessionClient()
    let provider = NetworkInterceptorProvider(client: client, store: store)
    let transport = RequestChainNetworkTransport(interceptorProvider: provider, endpointURL: url)
    return ApolloClient(networkTransport: transport, store: store)
  }()
}
```

This is largely based on Apollo documentation and Stack Overflow answers.
The middleware stack is provided by an _interceptor provider_, which is relatively easy to override using subclassing:

```swift
class NetworkInterceptorProvider: DefaultInterceptorProvider {
  override func interceptors<Operation>(for operation: Operation) -> [ApolloInterceptor]
  where Operation: GraphQLOperation {
    var interceptors = super.interceptors(for: operation)
    interceptors.insert(AuthInterceptor(), at: 0)
    return interceptors
  }
}
```

What the above code does is basically just saying "give me your default stack, put my new interceptor at the beginning, and return it."

The interceptor itself actually has more boilerplate than exsting code. Luckily, the function head was autocompleted by Xcode based on Apollo's type definitions:

```swift
class AuthInterceptor: ApolloInterceptor {
  func interceptAsync<Operation>(
    chain: RequestChain, request: HTTPRequest<Operation>, response: HTTPResponse<Operation>?,
    completion: @escaping (Result<GraphQLResult<Operation.Data>, Error>) -> Void
  ) where Operation: GraphQLOperation {
    if let tokens: Authentication.TokenPair = KeychainHelper.shared.read() {
      request.addHeader(name: "Authorization", value: "Bearer \(tokens.accessToken)")
    }

    chain.proceedAsync(request: request, response: response, completion: completion)
  }
}
```

I built a custom `User` struct that can be instantiated based on a GraphQL fragment called `UserDetails`.
Apollo for iOS generates separate types for each query and mutation, and all of their subtypes.
However, the same fragment is always decoded into the same underlying class, therefore by using the same fragment in different operations (like the initial login mutation and fetching session details), the same user struct can be decoded from the results of different operations.

```swift
import Foundation

struct User: Codable, Identifiable {
  var id: UUID
  var email: String
  var name: String
  var currency: Currency
  
  struct Currency: Codable {
    var code: String
    var exponent: Int
  }

  init(from details: UserDetails) throws {
    self.id = UUID(uuidString: details.id)!
    self.email = details.email
    self.name = details.name
    self.currency = Currency(code: details.actualCurrency.code, exponent: details.actualCurrency.exponent)
  }
}
```

By storing the abstract user struct on an `@EnvironmentObject`, the app can determine whether a login screen should be displayed, or the navigation stack for authenticated users:

```swift
import SwiftUI

@main
struct GouGouApp: App {
  @StateObject var authentication = Authentication()

  var body: some Scene {
    WindowGroup {
      if authentication.user != nil {
        ContentView()
          .environmentObject(authentication)
      } else {
        LoginView()
          .environmentObject(authentication)
      }
    }
  }
}
```

What all of this does not account for yet is refreshing access tokens.
This will require the for the application to check if the existing tokens are still valid.
