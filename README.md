[![CircleCI](https://circleci.com/gh/srodmiescie/tutaj.svg?style=shield)](https://circleci.com/gh/srodmiescie/tutaj)

# Tutaj

## Useful links

Official:

- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)

- [The Rust Programming Language](https://doc.rust-lang.org/nightly/book/ch01-00-getting-started.html)

- [Rocket](https://rocket.rs/v0.4/guide/)

- [Diesel](http://docs.diesel.rs/diesel/index.html)

- [Postgres Docker reference](https://docs.docker.com/samples/library/postgres/)

Others:

- [Referencee for containers](https://github.com/ghotiphud/rust-web-starter)

- [Referencee for containers 2](https://github.com/mario-orlicky/docker-starter)

## Prerequisites

- [Cargo Watch](https://github.com/passcod/cargo-watch) - run `cargo install cargo-watch`

## Run application

To run application use `cargo run`

To run application in watch mode use `cargo watch -x run`

## Run tests

To run all tests use `cargo test`

To run all tests in watch mode use `cargo watch -x test`

To run specific test use `cargo test <test_name>`

To run specific tests you can use part of the function name.

## Run app in docker

In main directory build app with `docker-compose build`.

Now you can run `docker-compose up`.

Server will automaticaly reload on any file change.

## Adminer

Adminer (formerly phpMinAdmin) is a full-featured database management tool written in PHP. Conversely to phpMyAdmin, it consist of a single file ready to deploy to the target server.

Adminer runs on adress `localhost:8080`.

Credentials:

| Label    | Value        |
| -------- | ------------ |
| System   | `PostgreSQL` |
| Server   | `db`         |
| Username | `postgres`   |
| Password | `postgres`   |
| Database | `postgres`   |