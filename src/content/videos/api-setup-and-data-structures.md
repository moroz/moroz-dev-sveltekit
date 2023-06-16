---
title: "Full Stack Elixir Part 1: API setup and data structures"
date: 2020-04-12
slug: api-setup-and-data-structures
youtube: 8MCmTiB3g6c
description: First video in my Full Stack Elixir tutorial with Phoenix, React, and Apollo Client. Setting up Phoenix with GraphQL.
tags:
  - Full Stack
  - Phoenix
  - React
---

### Transcript

Hi guys, in this series I'd like to show you how to build a full stack application using Elixir and React.
On the backend, we will first build a GraphQL API using the Phoenix framework and Absinthe, which is a full-fledged GraphQL implementation for Elixir, and we will be using Postgres for the database.
Then we will create a single-page application using React with hooks, and we will be talking to the API using apollo-boost.

The application we're going to build in this series will be a To-do list.
It's a very simple project in terms of features, but we can use it to cover many important topics that we can build upon in more advanced projects.

Now, in order to follow along with this tutorial, you need to have Elixir, Node, and Postgres installed on your machine.
You can check if you have Elixir installed by typing `iex --version`. Here you can see that I am running Elixir version 1.10.2.
Let's do the same for Node: type `node --version`. You can see that I am running Node version 12.16.0.
And now for Postgres: `psql --version`, and I have Postgres version 12.2 installed.

If any of these dependencies is missing, you should take a pause and install it before you go ahead.
The way you do it depends on the operating system you are using.
If you are running macOS, then you are in luck, because you can just install all of these using Homebrew.
And if you're using a distribution of GNU/Linux, such as Ubuntu or Debian, you should be able to find some versions of Elixir and Node in the system repositories, but they will likely be outdated, so when in doubt, please refer to official installation guides.

Another way to install Elixir and Node is using asdf, which is a universal version manager. If you'd like to install many versions of Node and Elixir, I highly recommend you take a look into asdf.
In the following part of this video, I'm goint to assume that you already have all of these installed.

Now, in order to create a new project with Phoenix, you first have to install the `mix phx.new` generator, which creates new applications. In order to install it, we will go to Hexdocs, into the documentation of Phoenix, and we will take a look into "Installation Guide", and down there in this document, there is a command that you have to copy and paste in your terminal.
Now, this command will install the `phx.new` generator, in fact I already had it installed. And we can now use this command we just installed to create a new project.
The command is `mix phx.new`, and then what follows is the name of the project, I will call it `todo_list`.
I press Enter, and the generator has just created several files for us, and then it offers to install dependencies.
These dependencies will be Elixir dependencies as well as dependencies for Node, because the Phoenix framework comes with batteries included and it has Webpack configured.
Now, while these dependencies are downloading, we can go to a new tab, and take a look at the files we have just created.
I open them up in Emacs, but you can use any editor you like.
The structure of a new Phoenix application is fairly simple.
There is the `assets` directory, which contains the front end files.
There is `config` we use for configuration.
Dependencies is where Elixir dependencies are installed. Lib is where our application lives, and the `priv` directory is for private files of the applications, and there is `test` for unit tests, `formatter.exs` is the configuration file for the Elixir code formatter. `mix.exs` is where we configure our dependencies.

We will now install two Elixir libraries that we will need to build a GraphQL API. Let's open up the `mix.exs` that is located in the project root. When we scroll down, almost to the bottom of the file, we will see a single function called `deps`. This is the function that defines our dependencies. It returns a list of tuples, and each one of these tuples represents a single library with a version number. So, let's jump to line 45, and after the tuple that requires `plug_cowboy`, I add a comma, and in the new line I add a tuple, the first element of the tuple is the name of the dependency as an atom, so first a colon and then `absinthe`, and as of this recording, the latest version of Absinthe is 1.4.16.
And another library I have to install is `absinthe_plug`, and this is a library that connects Absinthe, the GraphQL engine, with Phoenix. The latest version of this library is 1.4.7.
And if you are not sure what the latest version is, you can check on hex.pm, and in my case, I will type `absinthe_plug`, and we can see that the latest version is indeed 1.4.7.
So let's jump to the terminal and type `mix deps.get`, which will fetch the libraries, and before we start implementing a GraphQL API, we also need to create a database for our application.
So, the command to do this is `mix ecto.create`, and it will first compile our application, and in this command, `Ecto` is the name of a library that talks to the database, casts our data to correct types, and does validation, so it's kind of like an ORM, but we can't really say it's an ORM because Elixir is not an object-oriented programming language and therefore there are no objects.
So we can see that the database has already been created. Now that the database has been created, we can start crafting our GraphQL API. So I'll go back to the editor and open the file tree, and we can see that in the `lib` directory, in this `todo_list_web` subfolder, we have files that are related to our application's web layer, in this directory I will create a directory called `api`. And inside this directory, I will create a module called `schema.ex`. This file will be the main file of our GraphQL schema. It has to be an Elixir module, so I will type `defmodule TodoListWeb.Api.Schema`. This module has to be using `Absinthe.Schema`.
And in order to create the first meaningful Absinthe schema, we need to define at least one query. Absinthe doesn't use GraphQL for definition, it uses Elixir macros that get converted to actual GraphQL schemas.
So you write everything in Elixir code. There is a macro called `query`, where we define a "Hello, world" query, and this will be just a query that returns a string.
In order for this to work, we need to define a resolver.
There is a `resolve` macro that accepts a function that can have arity of either 2 or 3.
In this case we don't care about the arguments to the query, so we can just pass underscores for bindings.
Then we have to return a tuple, and the first element is the atom `:ok`, and the second one will be the actual string "Hello, world!". We save the file, and now we need to plug this schema into our router, so we go to the file `lib/todo_list_web/router.ex`. And down there, in the line 22, I'll uncomment this. There are two routes that we need to provide, one will be `POST /api`, and it should point to the schema that we just created. There is a plug called `Absinthe.Plug`, and it accepts a [keyword] list of options, the first option will be `schema: TodoListWeb.Api.Schema`.
Another route will be the GraphQL playground, which we probably want. I will do it on the same endpoint, but with GET method.
So, `get "/"`, the plug is called `Absinthe.Plug.GraphiQL`, because it used to provide GraphiQL, but obviously, we don't really want to use GraphiQL anymore, because it's slightly dated.
So we will define it in a moment. We have to give it a schema, it's the same module, and `interface: :playground`.
Oops. And let's see if this compiles.
I see. Here, I typed `Absinthe.Plug.GraphiQL`, but because this scope gets a second argument, `TodoListWeb`, then `TodoListWeb` will be prepended to the plug names. So I have to delete this argument.
Let's see if this compiles right now. It does.
I will spin up a server and when we go to the browser, at localhost:4000 we see the application, and if we go to `/api`, we should be able to see our GraphQL Playground.
On the right, we have our schema, and introspection shows that there is in fact this "hello" query just defined.
Let's try to run a query against the schema.
And it does indeed return "Hello, world!".

So the next step will be creating some data structures, so we can store our to-dos.
Phoenix has a lot of code generators that can simplify this task. There is one in particular that we will use right now, it's called "mix phx.gen.context", and it will create a context, which is a module that interacts with data, and I will call it "Todos".
The second argument of this generator command is the name of the actual schema, I will call it "Item".
The third argument is the table name, it will be the plural form of the schema, which is "items".
What follows is the list of columns in the table, and the first one will be the content, which is a string.
The last one will be a timestamp which indicates whether the to-do has been completed or not. I will call it `completed_at`, and the type is `utc_datetime`.
And we can go back to the editor and see the migration that was created.
It does indeed look OK, but I will add "null: false", so the column will not be nullable. And I will open the schema that we have just generated and it does look OK.
However, I just noticed that it does require us to specify `completed_at` which is not what we want.
So let's delete this and save it.
And now we can run the database migration, by using `mix ecto.migrate`, and the next thing that we're going to do is create a query that fetches all todo_items from the database.

In order to do this, let's go back to `schema.ex`, and in here I will create a new type, I will call it `:todo_item`, and let's define its fields.
First, I will add a field for its ID, in order to see its primary key, it's a non-nullable :id. This corresponds to `ID!` in GraphQL.
The next one will be content, which is a non-nullable string.
And the last one is a timestamp, but we don't really want to show the timestamp to the user, so we will make it `field :is_completed`, and it's a non-nullable boolean.
In order to return this from the database, we will need to provide a resolver function, and once again, we use the `resolve` macro, and if we give it a function with the arity of 3, then the first argument will be the object itself, and we can pattern-match on the `completed_at` timestamp.
The second argument is the arguments, which we don't need, and the third argument is the context.
Let's return `{:ok, !is_nil(completed_at)}`.
The next thing we have to do in order to actuall fetch these to-do items will be to provide another query.
I will do it in here. `field :todo_items`, and it's a non-nullable list of non-nullable todo items.
This corresponds in GraphQL to `[TodoItem!]!`
In here, let's provide a resolver, I will provide a separate resolver module for this later on. For now, let's just pass a function, and here we also don't care about the arguments and the context, so let's return `{:ok, TodoList.Todos.list_items()}`.
This is a function from the module that we have just generated using `mix phx.gen.context`. If you are not sure what this function does, you can open `iex -S mix`, and this will open a console.
In this console, we can see what this function does.
There's this module, TodoList.Todos, and it has the function `list_items/0`. We see that what this function does is query the database, selecting everything from the table `items`.
This looks like this is exactly what we want.
So let's jump back to the API Playground, and I will reload the window, so we can have a newer GraphQL schema. Oh, it's not here... what happened?
Let's see if it works. I will create a new query.
No, it's not working. This is peculiar.
Oh, I see. I think I misspelled this right now.
It should be `resolve`. Let's see if it helps.
It does indeed help, now we have this `todoItems` query and it does have a non-nullable content, non-nullable ID, and `isCompleted` as a boolean.
So, let's query all of these.
And we do get an empty array, because obviously, we don't have any to-do items yet.
So, in the next step, we can create some to-do items in the database. I suggest that you do it in IEx, because it's the simplest way to do it quickly.
We have this `TodoList.Todos` module, and I will alias it, so we can access it more quickly.
And now `Todos.create_item` and we pass it some arguments and I will create a to-do with the argument `"content": "Make a video about GraphQL, Elixir, and React"`.
And we see that Ecto has inserted this item into the database.
So if I go back to the GraphQL playground, we can now see that the query is already returning the correct item.
Another thing we would like to do is to create a mutation to create these items. In the schema file, I will use the mutation macro to create mutations.
So I will create a mutation called `create_todo_item`, and for now let's just return a boolean. It accepts just the text of this to-do item, and it has to be a non-nullable string.
And let's write a simple resolver, I will use the 2-arity version, and I pattern-match the content like this, we don't care about the second argument.
I will say `case`...
We have this TodoList.Todos module, I will alias this module right now so we can use it more easily in this module.
And down there I will substitute it with just `Todos`.
And in here, line 22, in the resolver, I will say `Todos.create_item(%{content: content})`.
If it returns `{:ok, %Todos.Item{}}`, then let's return `{:ok, true}`, and if it returns anything else, let's return `{:ok, false}`.
So now let's jump to the GraphQL Playground and I will open a new tab and create a mutation.
I will make this a bit bigger, so you can see it better.
`mutation createTodoItem`, and this accepts `$content` as a non-nullable string. And in here we create a to-do item.
`createTodoItem` with `content: $content`, and this field only returns a Boolean, which is a scalar. If we run it now, it will say that we didn't provide the content, so let's add this content right now in here.
And in this `content` variable I will write, let's say, `Write a decent mutation`. I run this mutation, and it does indeed create a to-do item, and we can see in the log that it did indeed create an item.
So if we jump back to the `todoItems` query we created before, it should return the new item.
And the last mutation we're going to need will be marking a to-do item as completed.
This will also be a mutation in this file, and let's say `:mark_todo_item_completed`.
Or we could also just say `toggle`, because sometimes we may want to do it with two states. When the to-do item is completed, we want to mark it as not completed, and if it's not completed, we want to make it completed.
So I'll call it `:toggle_todo_item`, and let's return the actual to-do item.
It may be nullable, because we may pass some invalid data to the mutation.
It will accept the ID of the to-do item, and it has to be a non-nullable ID.
And the resolver function will be a 2-arity function, we make it accept `%{id: item_id}`, and we don't care about the second argument.
In here, I suggest that we write a separate function to resolve it.
I will go back to the Todos context, and I will create a new function for this.
Define `toggle_todo_item_by_id`, and this will accept the to-do item's ID, and I will add a guard, so we can be sure that it is either a binary or a number, so we can see that it's an actual ID.
`when is_binary(todo_item_id) or is_integer(todo_item_id)`.
Inside this function, I will write it like this: `case Repo.get!(Item, todo_item_id) do`
I will write it like this: `Repo.get(...)`, so it returns nil instead of raising and exception.
If it returns nil, then we just return nil, and if it returns and item, let's update this item.
I will `update_item(item, ...)`. Ummm... I will write a separate function to just toggle it.
`def toggle_item`, if the item is completed... or rather if it has `completed_at: nil`, it means it's not completed, so I will update the item to say `completed_at` and let's pass the current time. I think you can get it like this: `DateTime.utc_now`. Yes, it works.
`DateTime.utc_now`, and if the item has some `completed_at`, it means that it's already completed, so we can pattern-match on the actual item, because this field can only have two possible values, either a timestamp or nil. 
So let's update the item with `completed_at: nil`.
And in here, I will just call `toggle_item(item)`.
In the resolver function, we will need to return a tuple, let's say `{:ok, Todos.toggle_todo_item_by_id}`... I'll rename it to `toggle_item_by_id` because it's more consistent with the rest of the API. We pass it the `item_id`.
OK, let's see if it works. We have the todoItems list. And I still haven't made a video about GraphQL, Elixir, and React, really, but I have already written a decent mutation, at least I think so, so I will write another one and see if I can mark this first to-do item as completed.
So I will reload the browser tab so we can see if the GraphQL schema has been updated. It is. I will go back to the new mutation, and I will make it accept an ID, which is non-nullable, and I will `toggleTodoItem` with the id of $id. And it should return content and isCompleted, and maybe the ID.
Let's see if it works. It complains that I didn't provide any ID. 
So, in the query variables below, I will add the ID of 2, and then run the query.
And we get an error. Let's see what happened.
I think I know what happened. This is because in the `toggle_item/1` I used the `update_item/2` function, which uses `Repo.update/2`, and `Repo.update/2` by definition returns a tuple. It either returns a tuple with :ok as the first element, or :error as the first element.
So, this `toggle_item/1` will return a tuple, and this branch would return nil, so I will convert it to return `{:ok, nil}`, and the `toggle_item` I will leave as is, so in the resolver, I remove the first :ok, because our function in the context will already return a tuple. And let's see if that works. Oops, what happened? It did work, but... OK, it did update it, I see what happened. The first time we ran this mutation, it has already updated the item, but we just didn't get the correct result from the mutation. And the second time we ran it, it returned nil, because it toggled it, and the third time over it also returns the correct `completed_at`.
I think this wraps up this video, and to wrap it up, we have created a new Phoenix application, we have installed absinthe and `absinthe_plug`, we have set up the Absinthe schema to work with the Phoenix router, and we have created one query and two mutations. Thank you guys, and if you liked this video, please like it and subscribe, and let me know if you have any suggestions in the comments below. Thank you!


