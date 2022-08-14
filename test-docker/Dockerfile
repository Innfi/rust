FROM rust:1.63.0-slim-buster as build

RUN User=root cargo new --bin tester
WORKDIR /usr/bin/app

COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release 
RUN rm ./target/release/deps/tester*

FROM rust:1.63.0-slim-buster
COPY --from=build /usr/bin/app/target/release/tester .
COPY ./conf ./conf

CMD ["tester"]