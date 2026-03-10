# Flight Booking Backend — Rust

A backend system for flight booking operations, built with Rust using Clean Architecture and Domain-Driven Design principles.

> **Note:** This is a learning project built by following a reference implementation and extending it with AI assistance. It covers DDD patterns, event-driven architecture, and Rust backend development practices.

---

## Architecture

Layered Clean Architecture with strict dependency inversion:

```
src/
├── domain/          # Entities, business rules, repository traits, domain events
├── application/     # Use case orchestration, DTOs, service interfaces
├── infrastructure/  # SeaORM repositories, Kafka publishers, Redis cache, JWT
├── presentation/    # HTTP handlers, request/response mapping
├── api/             # Route definitions
└── core/            # AppState, request context, shared config
```

Dependencies flow inward: `presentation → application → domain ← infrastructure`

---

## Technology Stack

| Layer | Technology |
|---|---|
| Language | Rust (Edition 2021) |
| Web Framework | Axum + Tokio |
| ORM | SeaORM |
| Database | PostgreSQL |
| Cache | Redis |
| Messaging | Kafka (rdkafka) |
| Auth | JWT + Argon2 password hashing |
| Migration | SeaORM Migration |
| Container | Docker, Docker Compose |

---

## Implemented Domains

### User
- Registration with domain validation (email uniqueness, password requirements, age check)
- Email verification flow with token expiry and resend rate limiting
- Login with failed attempt tracking and account lockout
- JWT token issuance + refresh token via Redis session store
- Domain rules as individual structs implementing `BusinessRuleInterface`

### Address
- Address entity with domain-level validation (phone format, recipient name)
- Association with user entity
- Domain events: AddressCreated, AddressUpdated, AddressDeleted

### Airport
- IATA code validation rule
- Full CRUD with repository abstraction

### Flight
- Domain rules: arrival must be after departure, origin ≠ destination, available seats ≤ total seats, check-in window validation
- Flight management APIs

### Booking
- Full booking lifecycle: Draft → Confirmed → Cancelled / Expired
- Payment tracking: Unpaid → Paid → Refunded
- **Optimistic locking** on updates via `version` field — prevents lost updates under concurrent requests
- Domain rules: booking code format, amount validation, contact info validation

### Passenger
- Passenger entity with validation (name, email, phone, DOB not in future, age range)
- Association with booking

### Check-in
- Check-in state: Pending → Completed
- Domain rules: baggage weight validation, check-in must be in pending state

### Boarding Pass
- Boarding pass generation linked to check-in

---

## Key Design Patterns

**Specification Pattern (BusinessRuleInterface)**
Each business rule is a separate struct implementing `check_broken() -> Result<(), DomainError>`.
Rules are composable, independently testable, and easy to extend without modifying entities.

```
domain/user/rules/
├── email_must_be_valid.rs
├── email_must_be_unique.rs
├── password_must_meet_requirements.rs
├── account_must_not_be_locked.rs
├── failed_login_limit_must_not_be_exceeded.rs
└── ... (15+ rules)
```

**Optimistic Locking**
Booking updates use `WHERE id = $1 AND version = $expected` with auto-increment on `version`.
Returns `OptimisticLockConflict` if another request updated the record concurrently.

**Event Publishing (Kafka)**
Domain events published via typed `EventPublisher` traits:
- `UserRegisteredEvent` → `user_registered` topic
- `UserActivatedEvent` → `user_activated` topic
- `UserLoggedInEvent` → `user_logged_in` topic
- `AddressCreatedEvent`, `AddressUpdatedEvent`, `AddressDeletedEvent`

**Request Context**
`RequestContextProvider` trait injected into repositories for audit fields (created_by, updated_by) without coupling domain to HTTP layer.

---

## Database

Migrations managed via SeaORM Migration crate, located in `migration/src/`.

Implemented tables: `users`, `addresses`, `airports`, `flights`, `bookings`, `passengers`, `checkins`, `boarding_passes`

---

## Running the Project

### Prerequisites
- Rust (stable)
- Docker + Docker Compose

### Environment variables

Copy and configure `.env`:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/flight_db
REDIS_URL=redis://localhost:6379
KAFKA_BROKERS=localhost:9092
JWT_SECRET=your_secret_key
```

> ⚠️ Never commit `.env` with real credentials to version control.

### Run with Docker

```bash
docker-compose up --build
```

### Run locally

```bash
# Start infrastructure
docker-compose up postgres redis kafka -d

# Run migrations
cargo run -p migration

# Start server
cargo run
```

---

## Known Limitations

- No test coverage (unit or integration tests)
- No authentication on most endpoints beyond user/auth routes
- Kafka consumer not implemented — only event publishing
- No pagination on list endpoints
- Single-node only — no distributed tracing or metrics
