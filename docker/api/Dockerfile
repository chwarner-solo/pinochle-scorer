FROM rust:alpine as builder

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
RUN mkdir api frontend-builder
COPY Cargo.toml Cargo.lock ./
COPY frontend-builder/Cargo.toml frontend-builder
copy frontend-builder/src frontend-builder/src
COPY api/Cargo.toml api
COPY api/src api/src

RUN cargo build \
    --workspace \
    --release  \
    --target x86_64-unknown-linux-musl  \
    -p api

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /api
ENTRYPOINT ["/api"]