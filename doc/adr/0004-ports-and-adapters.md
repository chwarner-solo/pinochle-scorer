# ADR 4: Ports and Adapters (Hexagonal Architecture)

## Status
Accepted

## Context
To maximize testability and maintain a pure domain model, we want to ensure that domain code is decoupled from infrastructure and frameworks.

## Decision
The project will use the Ports and Adapters (Hexagonal) architecture. Domain code must not depend on any external libraries except for `thiserror::Error` for error handling. All infrastructure concerns (e.g., persistence, APIs) will be implemented as adapters.

## Consequences
- Domain logic is highly testable and reusable
- Infrastructure can be swapped or extended with minimal impact on domain code
- Slightly more boilerplate required for adapter interfaces
