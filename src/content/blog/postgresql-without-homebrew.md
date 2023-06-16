---
title: "PostgreSQL without Homebrew on macOS"
date: 2022-08-02
slug: postgresql-without-homebrew
summary: |
  In this post I describe why I decided not to use PostgreSQL
  from Homebrew and how you can use a Dockerized image instead.

tags:
  - MacOS
  - PostgreSQL
  - Docker
---

## Rationale

If you have ever worked with PostgreSQL on macOS, chances are you have used the version shipped by Homebrew.
While Homebrew is definitely one of the simplest ways to manage software on macOS and Linux (yes, Homebrew works on Linux just as fine), the development experience with PostgreSQL installed with Homebrew can be pretty bad.

The first issue you may encounter when using PostgreSQL from Homebrew is that sometimes when you turn off your computer, PostgreSQL just won't start when you boot it up again.
This can be solved by deleting a file called `postmaster.pid`, which is the file where the PostgreSQL service stores the identifier of the server process.
This is somewhat problematic, and every time I encounter this problem, I have to specifically search for [this StackOverflow thread](https://stackoverflow.com/questions/36436120/fatal-error-lock-file-postmaster-pid-already-exists) to see where I can try to look for that file.

Another, potentially more serious issue, is that with some commands, Homebrew automatically upgrades all installed packages.
Unlike apt or pacman, it doesn't give you a heads-up about the packages it's about to upgrade.
This is fine with most software, but with PostgreSQL, the data format is slightly different for each major version.

Once Homebrew upgrades PostgreSQL to the next major version, you will find that your existing data folder is now incompatible with the version installed, and that you have to upgrade it (which is difficult) or purge all data folders and create a new cluster.
This is made even more problematic by the fact that PostgreSQL's data is spread across multiple folders and differs by system architecture.
It is even worse when it happens in the middle of a busy workday.

## Installing Docker

One solution to this problem is using a Dockerized version of PostgreSQL.
The easiest way to install Docker is using Homebrew:

```shell
brew install --cask docker
```

This will install the Docker desktop application using the official installer.
After the installation, you will need to run the newly installed desktop app to configure your system.

## Setting up services

Then, create a `docker-compose.yml` in your home directory with the following contents:

```yaml
version: '3.9'

services:
  pg:
    ports:
      - 5432:5432
    image: postgres:14.4-bullseye
    restart: unless-stopped
    environment:
      POSTGRES_USER: postgres
      POSTGRES_DB: postgres
      POSTGRES_PASSWORD: postgres
    volumes:
      - "./.data/pg:/var/lib/postgresql/data"
```

This service definition tells docker to run PostgreSQL 14.4 using the [official Docker image](https://hub.docker.com/_/postgres), with the user `postgres`, password `postgres`, and default database `postgres`.
The version is explicitly specified in order to avoid unplanned upgrades.
The database server will be listening on port 5432.
All persistent storage will go into `~/.data/pg`, so in case you break something, you can just remove the whole directory and start from scratch.
Finally, `restart: unless-stopped` tells docker to restart the container after reboot, unless you explicitly stop it.

You can start the services in the background using docker-compose:

```shell
docker compose -f $HOME/docker-compose.yml up -d
```

## Using command line tools

With this approach, we don't need the `postgresql` Homebrew package installed on the system, and the database server will run just fine.
However, this way we do not get the commonly used command line tools like `psql`, `pg_dump`, `pg_restore`, `createdb`, and the like.
These are shipped in a separate package called `libpq`.
To install and link these programs:

```shell
brew install libpq
brew link --force libpq
```

## Connecting

When PostgreSQL is running inside of a Docker container, we cannot connect to the server using a UNIX socket, and we need to resort to a network connection.
In order to run `psql` with the host `localhost` and user `postgres`, we would have to use a command like this:

```shell
PGPASSWORD=postgres psql -U postgres -h localhost
```

Luckily, all default connection parameters can easily be set using environment variables.
You are going to need at least the following:

```shell
export PGHOST=localhost
export PGUSER=postgres
export PGPASSWORD=postgres
```

You can teach your shell to automatically set these variables for each session by putting these lines in your shell configuration file (`~/.bashrc` if you're using bash, `~/.zshrc` if you are using zsh), or using a tool like [direnv](https://direnv.net/).
Once you source this file, you can connect to PostgreSQL without any additional parameters:

```
karols-mbp:~$ psql
psql (14.4)
Type "help" for help.

postgres=# select version();
                                                              version
-----------------------------------------------------------------------------------------------------------------------------------
 PostgreSQL 14.4 (Debian 14.4-1.pgdg110+1) on aarch64-unknown-linux-gnu, compiled by gcc (Debian 10.2.1-6) 10.2.1 20210110, 64-bit
(1 row)
```
