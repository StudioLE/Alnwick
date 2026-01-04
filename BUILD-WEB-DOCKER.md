# Build the web app from source with Docker

The docker implementation provides a complete developer environment using `dx serve`.

## `Dockerfile`

The [Dockerfile](docker/dioxus/Dockerfile) is configured to do the following:

1. Fetch the `bulma` CSS framework and `font-awesome` icons from `npm`.

2. Install [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) and prepare a build recipe for the cargo dependencies.
This caches the dependencies to speed up subsequent builds.

3. Install the [dependencies](https://dioxuslabs.com/learn/0.7/getting_started/#linux) for running the `dx` CLI tool

4. Install the [`dx`](https://dioxuslabs.com/learn/0.7/tutorial/tooling#all-the-commands) CLI tool

5. Build the cargo dependencies with [cargo-chef](https://github.com/LukeMathWalker/cargo-chef).

6. Copy the `bulma` CSS framework and `font-awesome` icons from `npm`

7. Serve the web app using `dx serve`

## `docker-compose.yml`

The [docker-compose.yml](docker/dioxus/docker-compose.yml) is configured to do the following:

Use the root of the directory as the build context.

Use `docker/dioxus/target` as a volume for caching the cargo build artifacts.

Include `Cargo.toml` and the entire `packages` directory as volumes so `dx serve` detects changes and hot reloads.

Use your local `~/.cache/alnwick` and `~/.local/share/alnwick` as volumes for config and state

## Getting Started

1. Change to the `docker/dioxus` directory

```bash
cd docker/dioxus
```

2. Build the docker image

The first build of the docker image can take a while but subsequent builds are not required.

```shell
docker compose build alnwick
```

3. Serve the web app

Once built you can run the docker image to serve the web app.

Any changes to the source code will be hot reloaded.

```shell
docker compose run --rm alnwick
```
