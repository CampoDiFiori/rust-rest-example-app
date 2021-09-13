# CRUD app

This small app is an example of a REST API service written in Rust with a simple sqlite database behind. A helpful Postman collection with all the available endpoints can be found here:  `CRUD-Rust-app.postman_collection.json`

## Launching via Docker:

0. To launch it you need to have Docker installed: 
    - https://docs.docker.com/get-docker/

Then build the Docker image:
1. `docker build . -t crudapp`

If your config demands it, build it as root: `sudo docker build . -t crudapp`

Now you can run the Docker container:

2. `docker run -it -p 8080:8080 crudapp`

It will launch the app on port 8080. You can also change the port to e.g. 3000 like this: `docker run -it -p 3000:8080 crudapp`

## Launching natively:

Alternatively, if we don't want to use Docker, we can install the Rust toolchain and run it natively:

0. `curl https://sh.rustup.rs -sSf | sh`

Now we should have `cargo` installed, which will download the dependencies, compile everything and run it. To do that we use commands, which can also be found in the  `Dockerfile`:

1. `cargo install diesel_cli --no-default-features --features sqlite`
2. `diesel migration run`
3. `cargo build`
4. `cargo run`
