# Rust Space API Implementation

[![Build Status](https://travis-ci.org/coredump-ch/spaceapi.svg?branch=rust)](https://travis-ci.org/coredump-ch/spaceapi)

This is an implementation of the [SpaceAPI](http://spaceapi.net/) in Rust.
The old implementation in Python can be found in the `python` directory.

API Documentation: http://spaceapi.net/documentation

## Development

Use [Cargo](https://crates.io/) to build:

    $ cargo build

Then one can start the spaci api server:

    $ cargo run

You can also specify a different ip or port:

    $ cargo run -- -i 0.0.0.0 -p 1337

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

If you want to see logging, set the `RUST_LOG` env variable:

    $ cargo build
    $ RUST_LOG=warn target/debug/coredump_status

You can also show logs only from a specific crate:

    $ RUST_LOG=spaceapi_server=debug cargo run


## Storage Schema

We currently store data in the following two redis keys:

- people_present (integer)
- raspi_temperature (float)


## Docker Image

To build the docker image (which pulls `master` version from Github, not the
local codebase):

    $ docker build -t coredump/spaceapi:latest .

To run a new container from the image:

    $ export PORT=3000
    $ docker run -d -p 127.0.0.1:$PORT:3000 coredump/spaceapi

To stop it again:

    docker stop <container-id>


## Docs

You can build docs with `make docs`. Find them in the `target/doc/` directory.
