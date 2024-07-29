# SurrealDB

This guide outlines how to use SurrealDB in Rust projects.

Surrealdb provide a amazing dashboard user interface for their database called
Surreallist

SurrealDB offers an mazing dashboard user interface for its database
[Surrealist](https://surrealdb.com/surrealist)

## Resources

- Official Website: [SurrealDB](https://surrealdb.com)
- Github Repository:
  [surrealdb/surrealdb](https://github.com/surrealdb/surrealdb)
- Docker Repository: [surrealdb/surrealdb](https://hub.docker.com/surrealdb/surrealdb)

## Getting Started

### `start` command:

```sh
surreal start memory -A --auth --user root --pass root --bind 127.0.0.1:8080
```

Based on disk

```sh
surreal start file:database/database.db
```

usage: `surreal start [OPTIONS] [PATH]`

Arguments: - `[PATH]`: path used for store data, default: `memory`

Options:

- `-l` , `--log` <LOG>: The logging level for database server, [Level: none, full, error, warn, info, debug, trace]
- `-A`: Enable all capabilities
- `--auth`: Enable authentication

- `--user` <user> `--pass` [password]: set initial username and password to access the database

- `--bind` <address>: bind database on <address> given, default: 0.0.0.0:8000

### `sql` command:

```sh
surreal sql --endpoint memory --username --password root --ns namespace --db database --pretty
```

connect with specific ip:port:

```sh
surreal sql --ws://127.0.0.1:8000
```

usage: `surreal sql [OPTIONS]`

options:

- `-e`, `--endpoint` <ENDPOINT>: Remote database server url to connect [Default: ws://localhost:8000]
- `-u`, `--username` <USERNAME>: Database authentication username to use when
  connecting
- `-p`, `--password` <PASSWORD>: Database authentication password to use when connecting
- `--namespace` <NAMESPACE>: The selected namespace
- `--database` <DATABASE>: The selected database
- `--pretty`: Database response should be pretty printed

## Run using Docker

Start database in Docker and store the database data in disk:

```sh
docker run --rm --pull always --user $(id -u) -v $(pwd)/database:/database --name surrealdb -p 8000:8000 \
surrealdb/surrealdb:latest start --log trace --auth --user root --pass root \
file:/database/mydb.db
```

Connect database:

```sh
surreal sql --endpoint ws://localhost:8000 -u root -p root --ns test --db test
--pretty
```

## Run using Docker Compose

Create `.env` file to store environment variables for SurrealDB:

```.env
CONTAINER_NAME=surrealdb
DB_PASS=root
DB_USER=root
DB_PORT=8080
```

```docker-compose.yml
version: "3"

services:
  surrealdb:
    env_file:
      - .env
    image: surrealdb/surrealdb:latest
    container_name: $CONTAINER_NAME
    restart: always
    entrypoint:
      - /surreal
      - start
      - -A
      - --auth
      - --user
      - $DB_USER
      - --pass
      - $DB_PASS
    ports:
      - $DB_PORT:8000

    volumes:
      - ./data/database.db:/database.db
```

Start Docker Compose:

`docker-compose up -d`

##
