---
title: "Absinthe Tips and Tricks: Generic Types"
date: 2022-09-02
slug: absinthe-tips-and-tricks
tags:
  - Elixir
  - GraphQL
  - Macros
summary: |
  Over the years, I have developed a big collection of modules and
  macros that I copy and paste in all Elixir projects using Absinthe.
  It is high time I started sharing my experience with the community.
  Today I present a simple way to build &ldquo;generic&rdquo; types.
---

When building GraphQL APIs, regardless of the specific implementation or language, you will most likely find some repeating patterns of types that occur all over your projects.
A common example would be the type definition for a pagination response.
Unless you're doing Relay-style pagination, there is no standardized structure for GraphQL pagination responses, so the following is my own arbitrary approach:

```graphql
type Pokemon {
  id: ID!
  name: String!
  pokedexNo: Int!
}

type PokemonPage {
  data: [Pokemon!]!
  pageInfo: PageInfo!
}

# Input type to pass pagination params to the query
input PokemonPaginationParams {
  page: Int!
  pageSize: Int
  # generic filter param
  q: String
}

type PageInfo {
  page: Int!
  pageSize: Int!
  totalPages: Int!
  totalEntries: Int!
}
```

Translated into Absinthe's definitions, it would looks like this:

```elixir
defmodule PokemonWeb.Api.PokemonTypes do
  use Absinthe.Schema.Notation

  object :pokemon do
    field :id, non_null(:id)
    field :name, non_null(:string)
    field :pokedex_no, non_null(:integer)
  end

  object :pokemon_page do
    field :data, non_null(list_of(non_null(:pokemon)))
    field :page_info, non_null(:page_info)
  end

  input :pokemon_pagination_params do
    field :page, non_null(:integer), default_value: 1
    field :page_size, :integer
    field :q, :string
  end

  object :page_info do
    field :page, non_null(:integer)
    field :page_size, non_null(:integer)
    field :total_pages, non_null(:integer)
    field :total_entries, non_null(:integer)
  end

  object :pokemon_queries do
    field :paginate_pokemon, non_null(:pokemon_page) do
      arg(:params, non_null(:pokemon_pagination_params))
      resolve(&SomeResolver.filter_and_paginate_pokemon/2)
    end
  end
end
```

You may soon notice that two types are going to appear all over the place: the pagination types and pagination param types. 
This is because they play more or less the same role regardless of their placement in the application.

Using macros, you can very easily refactor those repetitive types using "generics."

Put the following macros in a separate helper module:

```elixir
defmodule PokemonWeb.Api.SchemaHelpers do
  use Absinthe.Schema.Notation

  defmacro pagination_fields(entry_type) do
    quote do
      field :page_info, non_null(:page_info)
      field :data, non_null(list_of(non_null(unquote(entry_type))))
    end
  end

  defmacro standard_pagination_params do
    quote do
      field :page, non_null(:integer), default_value: 1
      field :page_size, :integer
      field :q, :string
    end
  end
end
```

Then, you can rewrite your types using your new macros:

```elixir
defmodule PokemonWeb.Api.PokemonTypes do
  use Absinthe.Schema.Notation
  import PokemonWeb.Api.SchemaHelpers

  object :pokemon do
    field :id, non_null(:id)
    field :name, non_null(:string)
    field :pokedex_no, non_null(:integer)
  end

  object :pokemon_page do
    pagination_fields(:pokemon)
  end

  input :pokemon_pagination_params do
    standard_pagination_params()
  end

  object :page_info do
    field :page, non_null(:integer)
    field :page_size, non_null(:integer)
    field :total_pages, non_null(:integer)
    field :total_entries, non_null(:integer)
  end

  object :pokemon_queries do
    field :paginate_pokemon, non_null(:pokemon_page) do
      arg(:params, non_null(:pokemon_pagination_params))
      resolve(&SomeResolver.filter_and_paginate_pokemon/2)
    end
  end
end
```

If you use a library like <a href="https://hex.pm/packages/scrivener_ecto" target="_blank" rel="noopener noreferrer">scrivener_ecto</a> to paginate resources, you can also write a middleware module to format `%Scrivener.Page{}` structs as responses compatible with this structure.
To that end, I have written a module that you can find among my <a href="https://gist.github.com/moroz/ed91f2ba5900c46bfa9c525ae8017408" target="_blank" rel="noopener noreferrer">Github Gists</a>.
There are more gists on that page that I frequently use and that I can hopefully describe in more detail in the future.
