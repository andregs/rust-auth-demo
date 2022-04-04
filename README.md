# Auth in Rust

Based on tutorial from
https://betterprogramming.pub/structuring-rust-project-for-testability-18207b5d0243

### Install Postgres

```sh
sudo apt-get install libpq-dev
docker run --name auth-demo-db -e POSTGRES_PASSWORD=mysecretpassword -d -p 5432:5432 postgres
```

### Install sqlx & dasel CLIs

```sh
cargo install sqlx-cli --no-default-features --features native-tls,postgres
curl -sSLf "$(curl -sSLf https://api.github.com/repos/tomwright/dasel/releases/latest | grep browser_download_url | grep linux_amd64 | cut -d\" -f 4)" -L -o dasel && chmod +x dasel
mv ./dasel /usr/local/bin/dasel
```

### Create DB and the first migration file

```sh
sqlx database create --database-url $(dasel -f App.toml -r toml default.db.url)
sqlx migrate add -r new-credentials-table
```

Write the migration SQL scripts and execute it:

```sh
sqlx migrate run --database-url $(dasel -f App.toml -r toml 'default.db.url')
sqlx migrate run --database-url $(dasel -f App.toml -r toml 'test.db.url')
```

### Spin up Redis instance

```sh
docker run --name auth-demo-redis -d -p 6379:6379 redis
```
