# GUTP

A General User Text Persistence protocol.

## The Backend

This project aims to give a reference implementation for the GUTP backend service.

The frontend should be putting into another project. This backend could potentially support multiple frontends, or multiple different apps.

The initial idea is to support a forum app, a blog, a technical social app.

## Build

### build application wasm

```
cd gutp && spin build
```

Note in the above step the spin is the spin binary for the host machine, not the spin binary in the folder.
The one in the folder is for docker where we use ubuntu:20.04.

### build application docker image

```
./build_app_image.sh
```

### start framework

```
docker compose -f gutp-docker-compose-1node.yml up
```

## Development

Rebuild application image when code changed.

```
cd gutp && spin build
./build_app_image.sh
```

If there is DB change, following cleanup needed:

    Delete gutp-db_1-1
    Delete gutp-subnode_1-1

```
docker compose -f gutp-docker-compose-1node.yml stop
docker rm gutp-db_1-1
docker rm gutp-subnode_1-1
```
