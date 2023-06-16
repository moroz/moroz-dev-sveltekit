---
title: Using UUIDv6 or v7 as primary key in Ecto
author: Karol Moroz
date: 2023-01-22
slug: using-uuidv6-or-v7-as-primary-key-in-ecto

summary: |
  UUIDs are a great choice for primary keys in PostgreSQL.
  However, not all UUIDs were created equal.
  This post discusses the pros and cons of different UUID formats and explains
  how to use newer formats in Ecto for Elixir.
---

Yesterday I read a post on the Supabase blog called [Choosing a Postgres Primary Key](https://supabase.com/blog/choosing-a-postgres-primary-key).
The article discussed the topic of UUIDs in detail.

The acronym UUID stands for *Universally Unique IDentitfier* and refers to a 128-bit identifier that can be used to identify resources in databases and other computer systems.
In simpler terms, UUIDs are essentially really long numbers that are so long and random that we are unlikely ever to come across the same number, even if we generate them on different computers.
For that reason, they are sometimes called *Globaly Unique IDentifiers*, or GUID.

PostgreSQL has a native UUID data type and can do all sorts of operations on data with UUID primary keys.
The broad definition of UUID does not specify what exactly goes into the 128 bits of a UUID, and there are several variants to choose from.
So far, I have primarily used UUIDv4, which is essentially completely random.
Completely random identifiers have the advantage of being impossible to guess, but they are not a perfect choice for database primary keys.

If our primary key is an incremented integer (`serial` or `integer generated as identity`), we can get the last record in a table simply by adding `order by id desc limit 1`
to our query. In fact, this pattern is so common that many ORMs will do exactly this if we call `.last`.
Since UUIDv4 identifiers are completely random, they do not sort properly and are difficult for many databases to index.
We can still get the last record inserted into a table by ordering by creation timestamp (`order by inserted_at desc limit 1`), but if we do this often, we will also need to create an index on that column.

## New UUID formats

There are two proposed UUID formats designed to be sortable by generation time: UUIDv6 and UUIDv7.
UUIDv6 contains 48 bits of random data and is designed to be backward compatible with UUIDv1.
UUIDv7 contains 74 random bits and is significantly more secure than v7.
The [IETF memo on New UUID Formats](https://www.ietf.org/archive/id/draft-peabody-dispatch-new-uuid-format-04.html#name-uuid-version-7) recommends using UUIDv7 over v6 whenever possible.
In my projects, I will only use v7.

While PostgreSQL supports UUID identifiers out of the box, there is no way to automatically generate v7 UUIDs with the standard PostgreSQL distribution.
The [uuid-ossp](https://www.postgresql.org/docs/current/uuid-ossp.html) package in the standard PostgreSQL distribution can only generate UUID v1, v3, v4, and v5, but we can generate those at the application level.

## Using UUID primary keys in Ecto

In order to use UUID primary keys for all Ecto schemas in our Elixir application, we can define a base schema, as recommended in the [Schema Attributes](https://hexdocs.pm/ecto/3.9.4/Ecto.Schema.html#module-schema-attributes) section of the Ecto documentation.
When using Ecto with PostgreSQL, UUID columns can be defined in the schema either using the `:binary_id` basic type, or using a dedicated type module, such as `Ecto.UUID`.

In order to use v7 UUIDs, we need to find a library that can generate new UUID formats.
As of this writing, there are two libraries in the [Hex.pm registry](https://hex.pm/packages?search=uuid&sort=total_downloads) that can generate v6 and v7 UUIDs: [uniq](https://hex.pm/packages/uniq) and [uuid_utils](https://hex.pm/packages/uuid_utils).
I chose `uniq` because it is slightly newer and has total downloads than `uuid_utils`.

The function [Uniq.UUID.uuid7/1](https://hexdocs.pm/uniq/Uniq.UUID.html#uuid7/1) generates version 7 UUIDs:

```elixir
iex(1)> Uniq.UUID.uuid7
"0185d99a-fe93-7a40-b77f-1eb7895a9bef"
```

We can define a base schema module to make use of this function:

```elixir
defmodule MyApp.Schema do
  defmacro __using__(_) do
    quote do
      use Ecto.Schema

      # `:binary_id` does not support `:autogenerate` tuples
      # so we have to use `Ecto.UUID` or `Uniq.UUID` type.
      @primary_key {:id, Ecto.UUID, autogenerate: {Uniq.UUID, :uuid7, []}}

      # For foreign keys, we can use either `:binary_id` or UUID types
      @foreign_key_type :binary_id

      # parse timestamps as `DateTime` (for better ISO 8601 serialization)
      @timestamps_opts [type: :utc_datetime]
    end
  end
end
```

If you need to use UUIDv6, simply replace `:uuid7` with `:uuid6`.

Update all Ecto schemas in the application to `use` this module instead of `Ecto.Schema`:

```elixir
defmodule MyApp.Clients.Client do
  use MyApp.Schema

  # ID will be a version 7 UUID
  schema "clients" do
    field :name, :string

    # implicitly defines a `:user_id` column of the `:binary_id` type
    belongs_to :user, MyApp.Users.User

    # timestamps will be loaded as UTC `DateTime`s
    timestamps()
  end
end
```

If you are adopting an existing Phoenix application to use UUID primary keys, make sure to also instruct Ecto to use UUIDs in migrations.
Add this line to `config.exs`:

```elixir
config :my_app, MyApp.Repo, migration_primary_key: [name: :id, type: :binary_id]
```

That's all! From now on, all models in your project will be using UUID primary and foreign keys!
