Training vocabulary in any language for kids

# Run locally

WebUI:

```
cd src\WebUi\
pnpm run dev
```

WebApi:

```
cd src\WebApi\
pnpm run dev
```

# Docker build for Raspberry PI

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

## Run on PI

`docker run -d --restart=unless-stopped --name vocabu-larry -p 8101:8101 -p 8102:8102 -v /home/me/vocabu-larry:/app/config -e "VOCABULARRY_HOME=/app/config" vocabu-larry:pi`

# SSL certificates

- follow https://github.com/sagardere/set-up-SSL-in-nodejs
- make sure to grant read permissions to "users"
