---
title: Refactoring common Ecto queries with macros
date: 2021-06-26
slug: refactoring-common-ecto-queries-with-macros
summary: |
  This article describes quick and easy techniques to refactor commonly used SQL fragments
  in `Ecto.Query`, including `COALESCE`, `SUM`, `CONCAT`, and `CAST` clauses.
tags:
  - Elixir
  - Phoenix
  - Ecto
  - MySQL/MariaDB
  - metaprogramming
  - macros
---

## Introduction

A common requirement that appears in many custom-built systems is generating some sort of reports using complex SQL queries.
In some cases, the complexity of those queries can be truly mind-boggling.
Things can get even more difficult if the database you are dealing with comes from a legacy system.

When building said customer applications using Elixir, we can use `Ecto.Query`, an insanely powerful query builder that comes with Ecto.
In this article, I would like to present common select clauses I frequently need to implement in queries and how Elixir macros can make our code cleaner.

## `COALESCE`

When querying nullable columns, we often need to call `COALESCE` to ensure a column or expression always yields a value.
In fact, [coalesce/2](https://hexdocs.pm/ecto/Ecto.Query.API.html#coalesce/2) is a built-in macro in `Ecto.Query.API`.

For instance, if we wanted to generate a report describing the sales performance of our sales team, we could query our MariaDB database like so:

```sql
SELECT
	e.id,
	concat(e.first_name, ' ', e.last_name) AS `Employee name`,
	coalesce(round(sum(od.quantity * od.unit_price), 2), 0) AS `Sales total`,
	MIN(o.order_date) AS `First order date`,
	MAX(o.order_date) AS `Last order date`
FROM
	employees e
	LEFT JOIN orders o ON o.employee_id = e.id
	LEFT JOIN order_details od ON o.id = od.order_id
GROUP BY
	1
ORDER BY e.id;
```

Were we to convert this to an Ecto query, the first iteration could look something like this:

```elixir
from(e in Employee)
|> join(:left, [e], o in assoc(e, :orders))
|> join(:left, [e, o], od in assoc(o, :order_details))
|> group_by([e], e.id)
|> order_by([e], e.id)
|> select(
  [e, o, od],
  {
    e.id,
    %{
      full_name: fragment("CONCAT(?, ' ', ?)", e.first_name, e.last_name),
      sales_total: fragment("COALESCE(ROUND(SUM(? * ?), 2), 0)", od.quantity, od.unit_price),
      first_order_date: min(o.order_date),
      last_order_date: max(o.order_date)
    }
  }
)
|> Repo.all()
|> Map.new()
```

Knowing that `COALESCE` is available as a macro, we could rewrite it as follows:

```elixir
from(e in Employee)
|> join(:left, [e], o in assoc(e, :orders))
|> join(:left, [e, o], od in assoc(o, :order_details))
|> group_by([e], e.id)
|> order_by([e], e.id)
|> select(
  [e, o, od],
  {
    e.id,
    %{
      full_name: fragment("CONCAT(?, ' ', ?)", e.first_name, e.last_name),
      sales_total: fragment("ROUND(SUM(? * ?), 2)", od.quantity, od.unit_price) |> coalesce(0),
      first_order_date: min(o.order_date),
      last_order_date: max(o.order_date)
    }
  }
)
|> Repo.all()
|> Map.new()
```

## `ROUND`

Now that we have somewhat relieved the initial complexity of the query, we could try and further simplify that query by defining
a macro to round up the sum to two significant digits. Let's define a module to hold our query helpers:

```elixir
defmodule Northwind.QueryHelpers do
  @moduledoc """
  Macros to do common tasks in Ecto.Query
  """

  require Ecto.Query

  defmacro round(field, significant_digits \\ 2) do
    quote do
      fragment("ROUND(?, ?)", unquote(field), unquote(significant_digits))
    end
  end
end
```

Armed with this macro, we can further refactor the original query:

```elixir
import Northwind.QueryHelpers

def report_query do
  from(e in Employee)
  |> join(:left, [e], o in assoc(e, :orders))
  |> join(:left, [e, o], od in assoc(o, :order_details))
  |> group_by([e], e.id)
  |> order_by([e], e.id)
  |> select(
    [e, o, od],
    {
      e.id,
      %{
        full_name: fragment("CONCAT(?, ' ', ?)", e.first_name, e.last_name),
        sales_total: sum(od.quantity * od.unit_price) |> round() |> coalesce(0),
        first_order_date: min(o.order_date),
        last_order_date: max(o.order_date)
      }
    }
  )
  |> Repo.all()
  |> Map.new()
end
```

## Full name

If the name of our employees, customers, etc. are split across two columns, we can imagine we will often need to write out the `CONCAT` clause we see in the above snippet. In fact, we can write a macro to simplify that particular clause:

```elixir
defmacro full_name(schema) do
  quote do
    fragment("CONCAT(?, ' ', ?)", field(unquote(schema), :first_name), field(unquote(schema), :last_name))
  end
end
```

Thus, the above query can be further simplified to the following:

```elixir
import Northwind.QueryHelpers

def report_query do
  from(e in Employee)
  |> join(:left, [e], o in assoc(e, :orders))
  |> join(:left, [e, o], od in assoc(o, :order_details))
  |> group_by([e], e.id)
  |> order_by([e], e.id)
  |> select(
    [e, o, od],
    {
      e.id,
      %{
        full_name: full_name(e),
        sales_total: sum(od.quantity * od.unit_price) |> round() |> coalesce(0),
        first_order_date: min(o.order_date),
        last_order_date: max(o.order_date)
      }
    }
  )
  |> Repo.all()
  |> Map.new()
end
```

## Type casting

The currency I most commonly have to deal with, the New Taiwan Dollar, does not have floating-point denominations.
All amounts we ever query are integers. However, in MySQL, the `SUM` of integers is returned as a `DECIMAL`. The rationale behind this behavior is good, keeping rounding errors away. But when returning a value from a GraphQL resolver, we need to explicitly cast that number to an integer, otherwise Absinthe will throw an exception.
In order to convert an expression to integer, we need to wrap it in a call to `CAST`, as in: `CAST(SUM(o.total) AS SIGNED INT)`.

Let's define a macro to solve this issue:

```elixir
defmacro as_int(field) do
  quote do
    fragment("CAST(? AS SIGNED INT)", unquote(field))
  end
end
```

We can now use that macro in our query to make sure we always get integers
from our queries:

```elixir
import Northwind.QueryHelpers

def report_query do
  from(e in Employee)
  |> join(:left, [e], o in assoc(e, :orders))
  |> join(:left, [e, o], od in assoc(o, :order_details))
  |> group_by([e], e.id)
  |> order_by([e], e.id)
  |> select(
    [e, o, od],
    {
      e.id,
      %{
        full_name: full_name(e),
        sales_total:
          sum(od.quantity * od.unit_price)
          |> round()
          |> coalesce(0)
          |> as_int(),
        first_order_date: min(o.order_date),
        last_order_date: max(o.order_date)
      }
    }
  )
  |> Repo.all()
  |> Map.new()
end
```
