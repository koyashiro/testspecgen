FROM rust:1.58.1 AS build
WORKDIR /work
RUN apt-get update && apt-get install --no-install-recommends -y clang
COPY Cargo.toml /work/Cargo.toml
COPY Cargo.lock /work/Cargo.lock
COPY src /work/src
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /work/target/release/testspecgen /usr/local/bin/testspecgen
ENTRYPOINT ["/usr/local/bin/testspecgen"]
