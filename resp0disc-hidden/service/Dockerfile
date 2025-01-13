FROM rust:1.84-alpine

WORKDIR /app

COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src .
COPY ./config .
COPY ./templates .

RUN cargo build --locked --release
RUN cp ./target/release/resp0disc /bin/server

ENV APP_ENVIRONMENT="dev"

CMD ["/bin/server"]