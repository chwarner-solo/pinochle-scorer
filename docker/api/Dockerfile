FROM rust:alpine as builder

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml ./api/

RUN mkdir api/src && echo "fn main() {}" > api/src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl -p api
RUN rm -fr api/src

COPY api/src ./api/src
RUN touch api/src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl -p api

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /api
ENTRYPOINT ["/api"]