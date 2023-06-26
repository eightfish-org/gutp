# GUTP

A General User Text Persistence protocol.




## The Backend

This project aims to give a reference implimentation for the GUTP backend service. 

The frontend should be put into another project. This backend could potentially support multiple frontends, or multiple different apps.

The initial idea is to support a forum app, a blog, a technical social app.

## build

### build application wasm
```
cd gutp && spin build
```

### build application docker image
```
./build_app_image.sh
```

### start framework
```
docker compose -f docker-compose-1node.yml up
```

