---
title: Using Elixir Livebook for Phoenix Development
slug: using-elixir-livebook-for-phoenix-development
date: 2021-07-04
draft: true
summary: |
  This article describes my Elixir development workflow
  using Livebook.
---

## Introduction

In this post, I would like to quickly explain how you can use
[Elixir Livebook](https://github.com/elixir-nx/livebook) to get quick feedback when developing
application using [Elixir](https://elixir-lang.org/), [Phoenix](https://www.phoenixframework.org/),
and potentially even pure [Erlang](https://www.erlang.org) applications.

### Install latest Erlang and Elixir

First, let's install Livebook. You will need Elixir 1.12 or later. If you are working with
the [asdf version manager](https://asdf-vm.com/#/core-manage-asdf) (which you should, anyway),
you can install the latest versions of Erlang and Elixir using the following commands:

```bash
# Optional, will build Erlang with inline documentation
export KERL_BUILD_DOCS="yes"

asdf install elixir latest
asdf install erlang latest
```

Please be aware that you need to install asdf plugins for [Erlang](https://github.com/asdf-vm/asdf-erlang)
and [Elixir](https://github.com/asdf-vm/asdf-elixir) and their respective dependencies.
This will vary based on your operating system.

After the installation, take note of the precise versions installed and
set the newly installed versions as your global tool versions:

```bash
asdf global erlang 24.0.3
asdf global elixir 1.12.2-otp-24
```

## Install Livebook as `escript`

Install rebar3, hex, and Livebook:

```bash
mix local.hex --force
mix local.rebar --force
mix escript.install hex livebook
```

If you are using `asdf`, you will need to tell `asdf` to generate a shim for the `livebook` command.
This way you can call the command `livebook` anywhere in your console:

```bash
asdf reshim
```

## Up and running

If everything in the previous section has worked correctly, you should now be able to
run Livebook locally, like so:

```bash
$ livebook server
00:30:56.878 [info]     :alarm_handler: {:set, {:system_memory_high_watermark, []}}
[Livebook] Application running at http://localhost:8080/?token=mqrhw2emihrmb6utaovqzmq743umdq73
```

When you click on the link in the terminal, you should see Livebook's welcome screen,
like the one in the screenshot below:

<a href="/images/livebook/up-and-running.webp" target="_blank">
<img src="/images/livebook/up-and-running.webp" alt="JSON API running from an Elixir release" />
</a>

Now, click on the bright purple _New notebook_ button in the upper right corner of the page.
You can now pick a title for your new notebook. You can add Elixir and Markdown sections or
add inputs for data input. The specifics of Livebook usage have been covered in José Valim's
excellent screencasts covering [version 0.2](https://www.youtube.com/watch?v=MOTEgF-wIEI) and
the [initial announcement of Livebook](https://www.youtube.com/watch?v=RKvqc-UEe34), so I will
not cover them here.

## Connecting your application to Livebook

You can connect to any Elixir application, including one that is already running
(even your production server), as long as you know the **node name** and its **cookie**.
`cd` into your Phoenix application and run:

```bash
iex --sname livebook --cookie livebook -S mix
```

This command
