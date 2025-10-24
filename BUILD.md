# Build from source

## With Docker

1. Build the docker container

```bash
docker build . -t lineweights`
```

2. Run the docker container

```bash
docker run --rm -it -p 8080:8080 --name lineweights lineweights
```

## Without Docker

1. [Install Node with NPM](https://nodejs.org/en/download)

2. Install the dependencies from NPM

```bash
npm install
```

3. [Install `dx` and all dependencies for your platform](https://dioxuslabs.com/learn/0.6/getting_started/#)

4. Serve the dev environment

```bash
dx serve
```