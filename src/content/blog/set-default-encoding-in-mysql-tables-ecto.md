---
title: "Setting default charset in MySQL tables using Ecto"
date: 2021-02-06
slug: set-default-encoding-in-mysql-tables-ecto
---

Over the past few weeks, I have been working on implementing a Phoenix
application sharing a legacy database with an existing Node.js application.
The reason we decided to build the API as a separate application rather than
to expand the existing Node.js application was the poor development experience
with TypeORM, most importantly the awkward approach to preloading associations
and lack of transactional testing.

Our application is deployed to a VM on Google Compute Engine. The database
is a MySQL instance hosted on Google Cloud SQL. The database engine that came
with the instance is Oracle MySQL 5.7.25, which is a bit outdated in comparison
to the latest versions of MariaDB. During development, we have to take into account
some significant differences between the versions.

One issue I have encountered several times when working with aggregate functions
in MySQL is the typing for `SUM()`. If I understand it correctly, the `SUM()`
calculated for an integer column is of type `DECIMAL` (most likely to prevent integer
overflow errors) and needs to be coerced to be handled as an integer. To this end,
I would usually use the `CAST()` function. In MariaDB or latest versions of MySQL,
the usage is obvious:

```shell
Welcome to the MariaDB monitor.  Commands end with ; or \g.
Your MariaDB connection id is 37
Server version: 10.3.25-MariaDB-0ubuntu0.20.04.1 Ubuntu 20.04

Copyright (c) 2000, 2018, Oracle, MariaDB Corporation Ab and others.

Type 'help;' or '\h' for help. Type '\c' to clear the current input statement.

MariaDB [(none)]> select cast(42 as int);
+-----------------+
| cast(42 as int) |
+-----------------+
|              42 |
+-----------------+
1 row in set (0.001 sec)
```

However, this syntax will not work in older versions of MySQL, which do not recognize
the type `INT`:

```shell
Welcome to the MariaDB monitor.  Commands end with ; or \g.
Your MySQL connection id is 392
Server version: 5.7.25 MySQL Community Server (GPL)

Copyright (c) 2000, 2018, Oracle, MariaDB Corporation Ab and others.

Type 'help;' or '\h' for help. Type '\c' to clear the current input statement.

MySQL [(none)]> select cast(42 as int);
ERROR 1064 (42000): You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'int)' at line 1
```

Therefore, when dealing with older versions, I need to use the type named `UNSIGNED INT`:

```shell
MySQL [(none)]> select cast(42 as unsigned int);
+--------------------------+
| cast(42 as unsigned int) |
+--------------------------+
|                       42 |
+--------------------------+
1 row in set (0.000 sec)
```

Note that this syntax also works in newer versions of the MySQL engine.

---------

Another issue I have encountered when working with MySQL in a Phoenix application is
that older versions of MySQL have the default encoding of `latin1` (an extension of ASCII)
as opposed to 4-byte UTF-8 (`utf8mb4`). This causes errors when handling Unicode strings,
such as non-Latin scripts or Emoji.

The easiest way to solve this problem is to explicitly set the charset attribute when
creating the table. You can do this by passing the `options: "DEFAULT CHARSET=utf8mb4"`
attribute to `table/2` call in the initial migration. For instance, if you were to
create a `users` table, you could write a migration like the one below:

```elixir
defmodule MyApp.Repo.Migrations.CreateUsers do
  @moduledoc false
  use Ecto.Migration

  def change do
    create table(:users, options: "DEFAULT CHARSET=utf8mb4") do
      add :display_name, :string, null: false
      add :email, :string, null: false
      add :password_hash, :string

      timestamps()
    end

    create unique_index(:users, [:email])
  end
end
```
