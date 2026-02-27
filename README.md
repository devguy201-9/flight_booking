# Flight Booking Backend (Rust)

A backend flight booking system built with Rust, designed using Clean Architecture and domain-driven modular structure.

This project focuses on building a scalable and extensible backend foundation for a flight booking platform, starting from identity, authentication, and domain modeling layers.

---

## Architecture

The project follows a layered Clean Architecture approach:

```
src/
├── api/
├── application/
├── core/
├── domain/
│   ├── user/
│   ├── address/
│   ├── flight/
│   ├── booking/
│   ├── passenger/
│   ├── airport/
│   ├── checkin/
│   └── boarding_pass/
├── infrastructure/
├── presentation/
├── utils/
└── bin/
```

Layer responsibilities:

- Domain: Business entities, value objects, rules, repository traits
- Application: Use case orchestration
- Infrastructure: Database and external service implementations
- Presentation / API: HTTP layer and DTO mapping

Dependency inversion is achieved using Rust traits.

---

## Technology Stack

- Rust (Edition 2021)
- Axum
- Tokio
- PostgreSQL
- SQLx
- Serde
- UUID
- JWT Authentication
- Docker

---

## Implemented Modules

### User

- User registration
- Password hashing
- Login flow
- JWT token issuance
- Domain validation
- Repository abstraction

### Address

- Address entity modeling
- Association with user
- Domain-level validation
- Repository pattern implementation

### Authentication

- JWT-based authentication
- Middleware protection
- Configurable secret key
- Token validation pipeline

---

## Domain Structure

Each domain module is organized with:

- entity definitions
- business rules
- domain errors
- repository interfaces
- event and rule abstractions where applicable

This structure allows new modules (flight, booking, checkin) to be implemented without affecting existing layers.

---

## Database

Currently implemented:

- users
- addresses

Migrations are located in:

```
migration/
```

The schema is designed to support extension for flights, bookings, passengers, and check-in workflows.

---

## Running the Project

Clone the repository:

```
git clone https://github.com/devguy201-9/flight_booking.git
cd flight_booking
```

Configure environment variables in `.env`:

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/flight_db
JWT_SECRET=your_secret_key
```

Run with Docker:

```
docker-compose up --build
```

Or run locally:

```
cargo run
```

---

## Design Principles

- Clean Architecture
- Domain-driven module organization
- Trait-based repository abstraction
- Explicit error modeling
- Separation of business logic and infrastructure
- Extensible system foundation

---

## Project Status

The core identity and authentication layer is fully implemented.

The system architecture is prepared to support:

- Flight management
- Booking workflows
- Passenger management
- Check-in and boarding logic

The project is structured for incremental feature expansion while maintaining architectural integrity.
