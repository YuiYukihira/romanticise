# Romanticise

Romanticise is a (WIP) image saving thing. More explanation below.

# The pitch

My partner sends me lots of images, mostly of anime characters I like,
I want to save them but google photos pretty bad for discoverability, so it's more
like I save the photo and then into the void it goes.

I have a NAS I can use, but it's a bit of a pain to deal with, so I thought I'd roll
my own solution.

# Features

This is where Romanticise comes in, it has features boasting:

- Self-hosted: Run it yourself on your own NAS using [docker](https://www.docker.com/)
- Booru-like website: Rediscover your images easily with tags
- Automatic tagging: Use [saucenao](https://saucenao.com/) to automatically get tags from booru's
- Automatically get the highest quality version: Also use saucenao to find the origanal version and save that

Additionally, there are some other features that I'm plannning if I have time:

- Optional automatic upscaling, the original is big enough for you? Make it bigger with [waifu2x](https://github.com/nagadomi/waifu2x)
- Phone App: Save images directly to Romanticise with an app

# Building

Romanticise is intended to run in docker and uses [nix](https://nixos.org/) to build the
image. The [shell.nix](./shell.nix) file provides all the tools needed and will be loaded
by [direnv](https://direnv.net/) with [lorri](https://github.com/nix-community/lorri).
There's a hady makefile with all the common tasks.

## Prerequisites

To build the docker image you must have:

- docker
- nix

installed, everything else with be handled by nix for you.

## Setup

To setup the project simply do:

```shell
$ make setup
```

This'll start a postgres docker container which is required to build [sqlx](https://github.com/launchbadge/sqlx)
It'll also run `sqlx database create` for you to get sqlx ready.

## Build the docker image

To build the docker image run:

```shell
$ make load
```

This'll do a few things:

1. Run the [migrations](./migrations/)
2. Create a `sqlx-data.json` file to build offline without the DB
3. Create a `Cargo.nix` file using [crate2nix](https://github.com/kolloch/crate2nix)
4. Create the docker image at `build/romanticise.tar.gz`
5. Load the docker image

## Cleaning

To clean up the generated/built files run:

```shell
$ make clean
```

To stop the docker ran DB (will also remove the container):

```shell
$ make killdb
```

To wipe the project entirely to a clean state:

```shell
$ make wipe
```

##### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
