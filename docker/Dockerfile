FROM rust:1.63.0-slim-buster as build1

RUN User=root cargo new --bin tester
WORKDIR /usr/bin/app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo build --release
RUN rm /usr/bin/app/target/release/tester*

FROM rust:1.63.0-slim-buster as build2
RUN User=root cargo new --bin tester
WORKDIR /usr/bin/app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY --from=build1 /usr/bin/app/target ./target
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/bin/app
COPY --from=build2 /usr/bin/app/target/release/tester .
COPY ./conf ./conf

# WORKDIR ./tester
# COPY ./Cargo.toml ./Cargo.toml
# RUN cargo build --release
# RUN rm src/*.rs

# ADD . ./

# RUN rm ./target/release/deps/tester*
# RUN cargo build --release

# FROM debian:buster-slim
# WORKDIR /usr/bin/app
# COPY --from=build ./tester/target/release/tester .
# COPY ./conf ./conf

# WORKDIR /usr/bin/app

# COPY ./src ./src
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml

# RUN cargo build --release 
# RUN rm ./target/release/deps/tester*

# FROM debian:buster-slim
# WORKDIR /usr/bin/app
# COPY --from=build /usr/bin/app/target/release/tester .
# COPY ./conf ./conf

ENTRYPOINT ["/usr/bin/app/tester"]
