FROM rust:1.63.0-slim-buster as build

RUN User=root cargo new --bin tester
WORKDIR ./tester
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD ./src ./src
RUN rm ./target/release/deps/tester*
RUN cargo build --release


FROM debian:buster-slim

WORKDIR /usr/bin/app
COPY --from=build ./tester/target/release/tester .
COPY ./conf ./conf

EXPOSE 1330

ENTRYPOINT ["/usr/bin/app/tester"]
