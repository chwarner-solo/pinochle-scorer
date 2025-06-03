# ADR 0007: Use of Rust's Axum for Web Service

## Status
Accepted

## Context
We require a robust, type-safe, and performant web framework to expose our Pinochle Scorer application over HTTP APIs. Rust's [Axum](https://github.com/tokio-rs/axum) is a modern web framework built on Tokio, emphasizing ergonomics, modularity, and strong typing. It integrates well with async Rust, supports extractors for request/response types, and is widely adopted in the Rust ecosystem.

## Decision
We will use Axum as the primary web framework for building the API layer of the Pinochle Scorer application. This will handle HTTP routing, request/response handling, and middleware integration.

## Consequences
- The API layer will be structured around Axum routers and handlers.
- Enables use of async Rust for high concurrency and performance.
- Leverages Axum's ecosystem for middleware, error handling, and testing.
- Contributors should be familiar with Axum concepts (routing, extractors, middleware).

## Alternatives Considered
- **Actix Web**: Mature and performant, but Axum offers simpler async ergonomics and is more aligned with Tokio-based projects.
- **Warp**: Also built on Tokio, but Axum provides more explicit router composition and better ergonomics for request/response extraction.

## Related
- [Axum documentation](https://docs.rs/axum)
- [Tokio](https://tokio.rs/) for async runtime

---
2025 Pinochle Scorer Project
