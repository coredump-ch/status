# Coredump Space API Implementation

[![Build Status](https://travis-ci.org/coredump-ch/status.svg?branch=rust)](https://travis-ci.org/coredump-ch/status)

This is our implementation of the [Space API](http://spaceapi.net/) v0.13 in
Rust. We're using the [spaceapi](https://crates.io/crates/spaceapi) and
[spaceapi-server](https://crates.io/crates/spaceapi-server) crates to simplify
the implementation.

Space API Documentation: http://spaceapi.net/documentation


## Usage

To get the current SpaceAPI status object:

    GET /

To update a sensor value, send a PUT request to the sensor endpoint:

    PUT /sensors/<data_key>/ value=<value>

Examples for [curl](http://curl.haxx.se/) and [httpie](https://github.zoe3m/):

    $ curl -v -X PUT -d value=42.1337 http://127.0.0.1:3000/sensors/raspi_temperature/
    $ http --form put :3000/sensors/people_present/ value=3


## Development

### Basics

Use [Cargo](https://crates.io/) to build:

    $ cargo build

Then you can start the spaceapi server:

    $ cargo run

You can also specify a different ip or port:

    $ cargo run -- -i 0.0.0.0 -p 1337

### Logging

If you want to see logging, set the `RUST_LOG` env variable:

    $ cargo build
    $ RUST_LOG=warn target/debug/coredump_status

You can also show logs only from a specific crate:

    $ RUST_LOG=spaceapi_server=debug cargo run


## Datastores

### Redis

To use the redis storage start the redis server:

    $ redis-server

(...or start it using your favorite init daemon.)

You can access the database with the `redis-cli` tool:

    % redis-cli
    127.0.0.1:6379> SET people_present 1
    OK
    127.0.0.1:6379> GET people_present
    "1"
    127.0.0.1:6379> KEYS *
    1) "people_present"

### Schema

We currently store data in the following redis keys:

- people_present (integer)
- raspi_temperature (float)
- room_temperature (float)


## Docker Image

To build the docker image based on the current codebase:

    $ docker build -t coredump/spaceapi:latest .

If you want to test this using a redis database, first launch a redis container:

    $ docker run -d --name spaceapi-redis redis:3.0

Then launch a new container from the image:

    $ export PORT=3000
    $ docker run -d --name spaceapi -p 127.0.0.1:$PORT:3000 --link spaceapi-redis coredump/spaceapi

(If you don't need a datastore, you can also leave away the redis container and the `--link` argument.)

To stop it again:

    $ docker stop spaceapi
    $ docker stop spaceapi-redis
