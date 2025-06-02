# Pinochle Scorer

Pinochle Scorer is a Rust-based application for tracking and scoring games of Pinochle, following official rules and supporting advanced features such as bidding, trump declaration, meld, and trick tracking. The project uses Domain-Driven Design principles and is organized by domain, application, and infrastructure layers.

## Features
- Start and manage games and hands
- Enforce bidding rules and increments
- Track trump declaration and marriage
- Record melds and tricks
- Calculate and validate scores for both teams
- Robust error handling for invalid operations

## Project Structure
- `api/` - Main application code, including domain logic
- `api/src/domain/` - Core entities: Game, Hand, Player, Team, etc.
- `doc/requirements.md` - Current requirements extracted from tests
- `doc/adr/` - Architecture Decision Records

## Requirements
See [doc/requirements.md](doc/requirements.md) for detailed rules and logic enforced by the system.

## Architecture
See [doc/adr/](doc/adr/) for architecture decisions and rationale.

## Getting Started
1. Install Rust (https://www.rust-lang.org/tools/install)
2. Build and run the project:
   ```sh
   cargo build
   cargo test
   ```

## Contributing
Contributions are welcome! Please review the requirements and ADRs before submitting changes.

---

 2025 Pinochle Scorer Project
