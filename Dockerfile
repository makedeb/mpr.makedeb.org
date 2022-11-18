# Build.
FROM rust:1.65.0
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get dist-upgrade -y
RUN apt-get install npm -y
RUN cargo install just

WORKDIR /tmp/build
ADD Cargo.lock Cargo.lock
ADD Cargo.toml Cargo.toml
ADD justfile justfile
ADD media/ media/
ADD migration/ migration/
ADD src/ src/

RUN just build

# Package.
FROM ubuntu:22.04
COPY --from=0 /tmp/build/target/release/mpr /usr/bin/mpr
ENTRYPOINT ["/usr/bin/mpr"]
