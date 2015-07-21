.. image:: https://travis-ci.org/coredump-ch/spaceapi.svg?branch=rust
    :target: https://travis-ci.org/coredump-ch/spaceapi
Rust Space API Implementation
=============================

This is an implementation of the `SpaceAPI <http://spaceapi.net/>`_ in Rust.
The old implementation in Python can be found in the ``python`` directory.

API Documentation: http://spaceapi.net/documentation

Development
-----------

Use `Cargo <https://crates.io/>`_ to build::

    $ cargo build

Then one can start the spaci api server::

    $ cargo run

You can also specify a different ip or port::

    $ cargo run -i 0.0.0.0 -p 1337

To use the redis storage start the redis server::
    
    $ redis-server

(...or start it using your favorite init daemon.)

You can access the database with the ``redis-cli`` tool::

    % redis-cli 
    127.0.0.1:6379> SET people_present 1
    OK
    127.0.0.1:6379> GET people_present
    "1"
    127.0.0.1:6379> KEYS *
    1) "people_present"

Storage Schema
--------------

We currently store data in the following two redis keys:

- people_present (integer)
- raspi_temperature (float)

Docs
----

You can build docs with ``make docs``. Find them in the ``target/doc/``
directory.
