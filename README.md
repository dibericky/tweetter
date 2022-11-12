# Tweetter

This is a simple Twitter inspired Social Network with basic functionality made for study purpose.

The service is based on CQRS/event-sourcing architecture.

This repository contains 2 binary:
- read_model: Read Model, is the worker responsible for updating read models based on events
- web: Write Model, is the WebServer exposing (WIP) web API. Exposes WebAPI for exposing data through Web API from Read Model as well.

## How repository is structured

- members: contains the cargo workspaces
    - **web**: it's the entrypoint for the web server exposing API
    - **view**: contains the logic for the View
    - **repository**: contains the whole logic for communicate with the database. It's the only workspace knowing that we're using Postgresql. Contains also a submodule *event_sourcing* responsible for handling the Command & Aggregate of the CQRS pattern.
    - **read_model**: contains the binary responsible for updating the read model based on the event consumed
    - **message_broker**: contains the rabbitmq consumer e producer
    - **events**: contains the events and payload produced and consumed by the architecture
    - **controller**: it's the bridge between the web server and the rest of the architecture

## How to run

First, you need to run the docker-compose:

```bash
docker-compose up
```

This will start a rabbitmq container and a postgresql one.

To monitor rabbitmq navigate to: http://localhost:15672/ and enter `guest` as username and password.

Now that your postgresql db is up, you need to run migration to configure it properly:

```bash
cargo make setup-db
```

Note: this command uses diesel-cli. If you have not yet installed it, run:

```bash
./install.sh
```

this will create the tables you need. You can check them out using the following connection string:

```
postgres://username:password@localhost:8000/diesel_demo
```

Now you need to run the read_model worker:

```bash
cargo make run-read-model
```

Now that the read-model worker is listening, you can run the web server:

```bash
cargo make run-web
```
