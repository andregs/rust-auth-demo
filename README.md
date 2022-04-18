# Auth in Rust

This is a sample Rust microservice that can be deployed on Kubernetes.

This is a pet project for learning purposes that started from [this tutorial](https://betterprogramming.pub/structuring-rust-project-for-testability-18207b5d0243).

## tl;dr

Execute this demo app in [minikube](https://minikube.sigs.k8s.io/docs/start/) with:

```sh
minikube start --mount --mount-string=$PWD:/mnt/host --cpus=4
skaffold dev --trigger=manual --iterative-status-check
```

## What's inside

This project demonstrates how to create:
 - a [Rocket](https://rocket.rs/) web app
 - writing data to PostgreSQL & Redis
 - execute DB migrations on deploy
 - type safe SQL queries
 - deploying to Kubernetes
 - different profiles (default, local, test etc.)
 - externalized configuration
 - rest input validation
 - centralized error handling with [thiserror](https://github.com/dtolnay/thiserror/) and anyhow
 - tracer that logs messages with a http request ID for easy correlation
 - unit tests with [mocks](https://github.com/asomers/mockall/)
 - testing connected to a test db
 - http testing

https://user-images.githubusercontent.com/712092/163493261-8ed6b178-9eed-4417-a6cd-42e356b91d3e.mp4

## Mode details / How I did it

[Skaffold](https://skaffold.dev/docs/install/) will build the app, deploy it in the cluster, watch for code changes and cleanup on quit.

Use `kubectl` to generate k8s yaml files (and customize them according your needs).

```sh
kubectl create blablabla --dry-run=client -o yaml > ops/k8s/file.yaml
```

### Local Dev

Execute the app locally:

```sh
skaffold dev -m migrations --iterative-status-check --port-forward
APP_PROFILE=local cargo run
```

Execute the tests locally:

```sh
skaffold dev -m migrations --iterative-status-check --port-forward
DATABASE_URL=$(dasel -f App.toml -r toml test.db.url) sqlx database reset -y && cargo test
```

### Initialize skaffold project

```sh
skaffold init -k ./ops/k8s/storage/*.yaml --skip-build
```

### Deploy PostgreSQL + Redis and execute migrations

```sh
skaffold dev -m migrations --iterative-status-check --port-forward
```

Now you have the servers available for connection at localhost. Both dev & test databases get created and migrations executed.

### Install sqlx and dasel CLIs locally

```sh
sudo apt-get install libpq-dev
cargo install sqlx-cli --no-default-features --features native-tls,postgres
curl -sSLf "$(curl -sSLf https://api.github.com/repos/tomwright/dasel/releases/latest | grep browser_download_url | grep linux_amd64 | cut -d\" -f 4)" -L -o dasel && chmod +x dasel
mv ./dasel /usr/local/bin/dasel
```

You can use the [sqlx](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli#sqlx-cli) cli to create migration files and prepare the `sqlx-data.json`, and
[dasel](https://github.com/TomWright/dasel#dasel) cli is useful to parse toml files. For example, to validate your SQL queries against your local DB and generate the json file, you can execute:

```sh
DATABASE_URL=$(dasel -f App.toml -r toml local.db.url) cargo sqlx prepare -- --lib
```
