# Mousquedevs Achievement API

Full-featured and modular REST API built in **Rust** using the **Rocket v0.5** web framework, **Diesel v2** ORM, and a **PostgreSQL** database. Designed to run seamlessly under **Docker** and communicate with any frontend client (React, Vue, Svelte, Angular, mobile apps, etc.).

---

## 🎯 Key Features

- 🔐 **Secure Authentication (JWT)**: Registration, login, session validation via Rocket Request Guards (`AuthenticatedUser`, `AdminUser`).
- 🛡️ **Password Hashing**: Strong password hashing using the `bcrypt` algorithm.
- 🏆 **Comprehensive Achievement Management**:
  - Browse validated public achievements.
  - User proposals for new achievements (`pending` status).
  - Admin moderation (`approved`, `rejected` status).
  - Achievement unlocking by users with timestamp tracking (`obtained_at`).
  - Creation timestamp (`created_at`) persisted in database.
- 👤 **User Profiles**:
  - Personal profile with total points calculation and detailed list of unlocked achievements.
  - Public profile viewing for other users.
  - Bio and avatar URL updates.
- 🛠️ **Administration Panel (Admin Panel)**:
  - Moderation and review of achievement proposals.
  - User management and role updates (`user` / `admin`).
  - Manual grant and revocation of achievements for users.
  - Audit action history logs (`logs`).
- 📜 **Audit Logging System**: Automatic traceability of user and administrator actions stored in the `action_logs` table.
- 🔄 **Automated Database Migrations**: Diesel SQL migrations are embedded into the binary (`embedded_migrations`) and executed automatically on application startup.
- 🌐 **CORS & JSON Error Catchers**: Complete cross-origin request support for frontends and uniform JSON formatting for HTTP errors.

---

## 📁 Project Structure

```text
mousquedevs_achievement_api/
├── Cargo.toml                  # Rust project dependencies and metadata
├── Rocket.toml                 # Rocket server configuration
├── diesel.toml                 # Diesel CLI configuration
├── Dockerfile                  # Optimized multi-stage Docker build
├── docker-compose.yml          # Docker orchestration (Rocket API + PostgreSQL)
├── .env.example                # Environment variables template
├── migrations/                 # SQL migration scripts (up.sql / down.sql)
│   ├── 2026-09-15-000000_create_users_and_profiles/
│   ├── 2026-09-15-000001_create_achievements_and_user_achievements/
│   └── 2026-09-15-000002_create_action_logs/
├── tests/                      # Automated integration and unit tests
│   └── health_test.rs
└── src/
    ├── main.rs                 # Executable entry point
    ├── lib.rs                  # Rocket application setup, fairings, and route mounting
    ├── config.rs               # Environment variables and configuration management
    ├── schema.rs               # Strongly typed schema generated for Diesel
    ├── auth/                   # Authentication and security logic
    │   ├── mod.rs
    │   ├── jwt.rs              # JWT encode/decode and Rocket Request Guards
    │   └── password.rs         # Bcrypt password hashing and verification
    ├── db/                     # Database layer
    │   └── mod.rs              # DbConn pool and automatic migration fairing
    ├── errors/                 # Centralized error management
    │   ├── mod.rs              # ApiError enum and JSON serialization
    │   └── catchers.rs         # Rocket HTTP error catchers (400, 401, 403, 404, 422, 500)
    ├── fairings/               # Rocket middlewares
    │   ├── mod.rs
    │   └── cors.rs             # CORS header configuration
    ├── models/                 # Data models & DTOs
    │   ├── mod.rs
    │   ├── user.rs             # Users & auth payloads
    │   ├── profile.rs          # Profiles & user views
    │   ├── achievement.rs      # Achievements & proposals
    │   ├── user_achievement.rs # Many-to-many associations & unlock timestamps
    │   └── action_log.rs       # Audit action log entries
    ├── repositories/           # Data access layer (Diesel queries)
    │   ├── mod.rs
    │   ├── user_repository.rs
    │   ├── profile_repository.rs
    │   ├── achievement_repository.rs
    │   ├── user_achievement_repository.rs
    │   └── log_repository.rs
    └── routes/                 # API HTTP controllers
        ├── mod.rs              # CORS preflight and route aggregation
        ├── health.rs           # Health Check (`/api/health`)
        ├── auth.rs             # Authentication (`/api/auth/*`)
        ├── profiles.rs         # Profiles (`/api/profiles/*`)
        ├── achievements.rs     # Achievements (`/api/achievements/*`)
        └── admin.rs            # Administration (`/api/admin/*`)
```

---

## 🚀 Getting Started with Docker (Recommended)

The full environment (PostgreSQL database + Rust API with automatic migrations) starts with a single command:

```bash
docker compose up --build
```

The API is immediately available at: **`http://localhost:8000`**

*(The first user registered via `/api/auth/register` is automatically granted the `admin` role).*

To stop the environment:
```bash
docker compose down
```

---

## 💻 Local Setup (Without Docker)

1. **Prerequisites on Arch Linux**:
   ```bash
   sudo pacman -S postgresql-libs
   ```

2. **Start PostgreSQL** (e.g. via Docker):
   ```bash
   docker compose up db -d
   ```

3. **Create the `.env` file**:
   ```bash
   cp .env.example .env
   ```

4. **Run the application**:
   ```bash
   cargo run
   ```

5. **Run tests**:
   ```bash
   cargo test
   ```

---

## 📚 API Endpoint Documentation

### 1. Health & Diagnostics
| Method | Endpoint | Authentication | Description |
|---|---|---|---|
| `GET` | `/api/health` | Public | Checks API health status (`status: ok`) |

---

### 2. Authentication (`/api/auth`)
| Method | Endpoint | Authentication | Description |
|---|---|---|---|
| `POST` | `/api/auth/register` | Public | Registration (`username`, `email`, `password`). Returns JWT. |
| `POST` | `/api/auth/login` | Public | Login (`username_or_email`, `password`). Returns JWT. |
| `GET` | `/api/auth/me` | Bearer JWT | Retrieves full profile and achievements of the authenticated user. |

---

### 3. User Profiles (`/api/profiles`)
| Method | Endpoint | Authentication | Description |
|---|---|---|---|
| `GET` | `/api/profiles/<user_id>` | Public | Retrieves the public profile of a user and their unlocked achievements with timestamps. |
| `PUT` | `/api/profiles/me` | Bearer JWT | Updates profile information (`bio`, `avatar_url`). |

---

### 4. Achievements (`/api/achievements`)
| Method | Endpoint | Authentication | Description |
|---|---|---|---|
| `GET` | `/api/achievements` | Public | Retrieves list of all approved achievements. |
| `GET` | `/api/achievements/<id>` | Public | Retrieves achievement details by ID. |
| `POST` | `/api/achievements/propose` | Bearer JWT | Proposes a new achievement to administration (`status: pending`). |
| `POST` | `/api/achievements/obtain/<id>` | Bearer JWT | Unlocks / obtains an achievement for the authenticated user (`obtained_at`). |
| `POST` | `/api/achievements` | Admin | Directly creates an approved achievement (`title`, `description`, `points`). |
| `PUT` | `/api/achievements/<id>` | Admin | Modifies an existing achievement. |
| `DELETE` | `/api/achievements/<id>` | Admin | Permanently deletes an achievement. |

---

### 5. Administration Panel (`/api/admin`)
*(All these routes require a JWT token with the `admin` role)*

| Method | Endpoint | Authentication | Description |
|---|---|---|---|
| `GET` | `/api/admin/users` | Admin | Lists all registered users and their roles. |
| `PUT` | `/api/admin/users/<id>/role` | Admin | Updates a user's role (`role: "admin"` or `"user"`). |
| `GET` | `/api/admin/achievements` | Admin | Lists all achievements regardless of status. |
| `GET` | `/api/admin/achievements/pending` | Admin | Lists achievement proposals awaiting moderation (`pending`). |
| `PUT` | `/api/admin/achievements/<id>/status` | Admin | Approves or rejects an achievement proposal (`status: "approved"` or `"rejected"`). |
| `POST` | `/api/admin/users/<user_id>/grant/<achievement_id>` | Admin | Manually grants an achievement to a user. |
| `DELETE` | `/api/admin/users/<user_id>/revoke/<achievement_id>` | Admin | Revokes an achievement from a user. |
| `GET` | `/api/admin/logs?limit=100` | Admin | Retrieves system audit action logs. |

---

## 🔒 Using JWT Authentication in Requests

To call a protected route, include the following HTTP header:

```http
Authorization: Bearer <your_jwt_token>
```
