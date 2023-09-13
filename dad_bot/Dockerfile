FROM rust:1.70.0 as build

RUN cargo new --bin dad_bot
WORKDIR /dad_bot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build -r
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/dad_bot*
RUN cargo build -r

FROM debian:latest

WORKDIR /dad_bot
COPY --from=build /dad_bot/target/release/dad_bot .

CMD ["./dad_bot"]