FROM rust:1.32-stretch
COPY ./Cargo.toml /Cargo.toml
COPY ./Cargo.lock /Cargo.lock
COPY ./src /src

RUN cargo build
EXPOSE 8000
ENTRYPOINT /target/debug/rust-ci-test
