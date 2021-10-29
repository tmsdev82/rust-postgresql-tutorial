# Rust connect to PostgreSQL example

This repository shows example code for connecting to a PostgreSQL database using the Rust programming language.

Included is a docker-compose file for running a PostgreSQL database instance.

There is an article on my blog site explaining the code here: [TMS DEV Blog - PostgreSQL database with Rust: basic how to](https://tms-dev-blog.com/postgresql-database-with-rust-how-to/)

## Start and run

You have to have docker and docker-compose installed to be able to use the docker-compose.yml.

To start the PostgreSQL instance:

```bash
docker-compose up
```

To start the program:

```bash
cargo run
```