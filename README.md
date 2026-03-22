Training vocabulary in any language for kids

# Run locally

## Development 

Frontend:

```
cd src\WebUI\
trunk serve
```

WebApi:

```
cd src\WebApi\
cargo run
```

## Testing

```
cargo xtask local
```

This builds the Rust/Yew frontend into `src\WebApi\public`, builds the Rust backend,
starts it in production mode on `8101`, and opens the browser automatically.

It simulates the production setup locally:

- Rust serves both the API and the built frontend
- `NODE_ENV=production` is set
- `VOCABULARRY_HOME` points at the repository root so dictionaries and score files are found
- logs are written to `logs/` under `VOCABULARRY_HOME` unless `VOCABULARRY_LOG_DIR` is set

# Acceptance tests

The repository now includes a Rust-only acceptance test harness under `tests/`.

Build the acceptance artifacts and run the scenarios with:

```powershell
cargo xtask acceptance
```

- builds the Yew frontend with `trunk build --release`
- builds the backend binary once
- builds the acceptance runner
- executes the acceptance runner against the prebuilt backend binary

# Docker

The Docker image is now Rust-only during the build as well: Trunk builds the Yew frontend, and the backend image serves the generated static files.

# Docker build on Windows for Raspberry PI

## Setup a builder

`docker buildx create --name pi-builder --use`

## Build

`docker buildx build --platform linux/arm64 -t vocabu-larry:pi . --load`

# Deploy without registry

Save on development system

`docker save vocabu-larry:pi --output vocabu-larry.tar`

Copy over to PI, log in and run:

`docker stop vocabu-larry` 
`docker rm vocabu-larry` 
`docker load --input vocabu-larry.tar`

# Docker build on Raspberry PI itself

```
sudo docker build -t vocabu-larry .
```

```
sudo docker stop vocabu-larry
sudo docker rm vocabu-larry
```

## Run on PI

`docker run -d --restart=unless-stopped --name vocabu-larry -p 8101:8101 -p 8102:8102 -v /home/me/vocabu-larry:/app/config -e "VOCABULARRY_HOME=/app/config" vocabu-larry:pi`

If you want logs somewhere else, also set `VOCABULARRY_LOG_DIR`, for example `-e "VOCABULARRY_LOG_DIR=/app/config/logs"`.

# SSL certificates

- follow https://github.com/sagardere/set-up-SSL-in-nodejs
- make sure to grant read permissions to "users"

# Design & Implementation

- the backend now lives in `src/WebApi`
- the frontend now lives in `src/WebUI`
- run it with `cd src\WebApi && cargo run`
- it serves the built frontend in production from `public/`
- by default it listens on `8101` and `8102`; set `VOCABULARRY_HTTP_PORT` and `VOCABULARRY_HTTPS_PORT` to change that
- local development keeps using `../../` as the repository root for dictionaries and scores
- production uses `NODE_ENV=production` together with `VOCABULARRY_HOME`
- request errors are logged into one daily `.log` file inside `VOCABULARRY_LOG_DIR` or `logs/` under `VOCABULARRY_HOME`
- the main screen includes a `Logs` page that shows backend log contents in the browser
- HTTPS is enabled automatically when `selfsigned.crt` and `selfsigned.key` exist in `VOCABULARRY_HOME`
