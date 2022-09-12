FROM rust AS builder

RUN apt-get update && apt-get install musl-tools --no-install-recommends -y && \
  rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src
COPY Cargo.toml Cargo.lock ./

RUN mkdir src/ && \
  echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
  RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl && \
  rm -f target/x86_64-unknown-linux-musl/release/deps/fesghel*

COPY . ./

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

WORKDIR /fesghel
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/fesghel .
COPY ./config ./config
CMD ["./fesghel"]
