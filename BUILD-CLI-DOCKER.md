# Build the CLI from source with Docker

The `docker/cli/docker-compose.yml` file contains a recommended setup with:

- CLI tunneled through a VPN using [gluetun](github.com/qdm12/gluetun)
- A [Caddy](https://caddyserver.com/) file server for browsing and serving the downloaded podcasts

1. Change to the `docker/cli` directory

```bash
cd docker/cli
```

2. Create a `.env` file based on `.env.example`. Fill in the required values.

```shell
cp .env.example .env
micro .env
```

4. Run the help command with `alnwick`

The `gluetun` VPN will automatically start as well.

```bash
docker compose run --rm  alnwick --help
```

After the command has run the `alnwick` container will stop but the `tunnel` container will remain running.

Stop the tunnel container with:

```bash
docker compose down tunnel
```

5. Scrape a podcast

```bash
docker compose run --rm alnwick scrape irl https://irlpodcast.org
```

6. Download episodes

Downloading all episodes of a podcast will take a while and consume a lot of data and disk space.
It's therefore recommended to download in batches using the filtering capabilities, check the syntax with:

```bash
docker compose run --rm alnwick download --help
```

Download all episodes from 2019:

```bash
docker compose run --rm alnwick download irl --year 2019
```

Download all episodes of season 2:

```bash
docker compose run --rm alnwick download irl --season 2
```

7. Start the caddy file server

Start the caddy file server to browse the episodses:

```bash
docker compose up caddy
```

Open your browser at http://localhost:4000

8. Create RSS feeds

Create RSS feeds for all downloaded episodes of a podcast:

```bash
docker compose run --rm alnwick emulate irl
```
