FROM rust:1.60
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
ENTRYPOINT ["sqlx"]
# see migrations on skaffold.yaml
