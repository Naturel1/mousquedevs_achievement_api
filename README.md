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
- 📜 **Clean Local File Logging System**: Structured logging system writing directly to local text files (`logs/app.log`) and console output with timestamps, log levels (`INFO`, `WARN`, `ERROR`), and clear messages.
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
│   ├── 2026-09-15-141429-0000_remove_points/
│   └── 2026-09-16-135500_add_unique_constraint_to_username/
├── tests/                      # Automated integration and unit tests
│   └── health_test.rs
├── logs/                       # Local text log files (ignored by Git)
│   └── app.log
└── src/
    ├── main.rs                 # Executable entry point
    ├── lib.rs                  # Rocket application setup, fairings, and route mounting
    ├── config.rs               # Environment variables and configuration management
    ├── logging.rs              # File and console logging system configuration (fern + log)
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
    │   ├── cors.rs             # CORS header configuration
    │   └── logger.rs           # HTTP request & response logging fairing
    ├── models/                 # Data models & DTOs
    │   ├── mod.rs
    │   ├── user.rs             # Users & auth payloads
    │   ├── profile.rs          # Profiles & user views
    │   ├── achievement.rs      # Achievements & proposals
    │   └── user_achievement.rs # Many-to-many associations & unlock timestamps
    ├── repositories/           # Data access layer (Diesel queries)
    │   ├── mod.rs
    │   ├── user_repository.rs
    │   ├── profile_repository.rs
    │   ├── achievement_repository.rs
    │   └── user_achievement_repository.rs
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

### 🗄️ Database Migrations with Docker

All SQL migration scripts located in `migrations/` are embedded into the application binary at compile-time via `diesel_migrations`.

> **Note on Diesel CLI & Docker:**  
> Running `docker compose exec api bash` to use Diesel CLI is **not recommended** because `diesel_cli` is not pre-installed in the Docker container (compiling it takes several minutes) and the production image is minimal.  
> The safest and cleanest approaches are either **letting the API handle migrations automatically** or **running Diesel CLI from your host machine** (connecting via the exposed port `5432`).

#### Generate a migration with Diesel
```bash
diesel migration generate <name>
```

#### 1. Automatic Execution on Startup (Safest & Recommended)
When the Docker container starts, the Rocket ignition fairing (`MigrationFairing`) automatically applies all pending migrations against the database. No manual intervention or Diesel CLI installation is required.

#### 2. Running Migrations Once Docker is Running
If you need to apply, revert, or manage migrations after starting the Docker containers:

- **Option A: Restart the API container (Recommended)**
  If new migration files were added or modified, restarting the container re-triggers the embedded migration check:
  ```bash
  docker compose restart api
  ```
  *(If using the production multi-stage `Dockerfile`, rebuild first: `docker compose up --build -d`)*

- **Option B: Using Diesel CLI from the host machine**
  Since PostgreSQL port `5432` is exposed to the host, you can run Diesel CLI locally to manage migrations and keep `src/schema.rs` synced with proper user permissions:
  ```bash
  # Run pending migrations
  diesel migration run --database-url="postgres://postgres:postgres@localhost:5432/maadb"

  # Check migration status
  diesel migration list --database-url="postgres://postgres:postgres@localhost:5432/maadb"

  # Revert the latest migration
  diesel migration revert --database-url="postgres://postgres:postgres@localhost:5432/maadb"

  # Redo (revert & re-run) the latest migration
  diesel migration redo --database-url="postgres://postgres:postgres@localhost:5432/maadb"
  ```

- **Option C: Direct SQL execution via Docker `psql`**
  Execute any SQL migration file directly into the running database container without extra host tools:
  ```bash
  docker compose exec -T db psql -U postgres -d maadb < migrations/<migration_name>/up.sql
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

---

## 📝 Logging System (Local Text Files)

The application features a clean, high-performance file-based logging architecture using `fern` and `log`. Logs are **not** stored in PostgreSQL database tables, but written directly to local text files on disk as well as to standard console output.

### ⚙️ Configuration (`.env`)
```bash
# Log level: trace, debug, info, warn, error (default: info)
LOG_LEVEL=info

# Target text log file path (default: logs/app.log)
LOG_FILE=logs/app.log
```

### 📄 Log Format
Each log entry is timestamped and structured as follows:
```text
[YYYY-MM-DD HH:MM:SS] [LEVEL] [TARGET] Message
```

#### Example Output (`logs/app.log`):
```text
[2026-09-16 14:26:53] [INFO] [app::logging] Logging system initialized successfully. Writing logs to: logs/app.log
[2026-09-16 14:27:01] [INFO] [app::fairings::logger] --> POST /api/auth/login (client: 127.0.0.1)
[2026-09-16 14:27:01] [INFO] [app::routes::auth] User 'alexandre' (id: 1) successfully logged in
[2026-09-16 14:27:01] [INFO] [app::fairings::logger] <-- POST /api/auth/login [200 OK]
[2026-09-16 14:27:15] [INFO] [app::routes::achievements] User 'alexandre' (id: 1) unlocked achievement 'First Steps' (id: 2)
[2026-09-16 14:27:42] [WARN] [app::errors] 404 Not Found on GET /api/achievements/999: Achievement with identifier 999 does not exist
```

### 🔍 Features Covered
- **HTTP Request & Response Fairing (`RequestLogger`)**: Automatically records all incoming HTTP requests (HTTP method, URI, remote client IP) and outbound responses (status code).
- **Authentication & Security Events**: Logs user registrations, logins, failed authentication attempts, and authorization denials.
- **Achievement & Moderation Lifecycle**: Logs proposals, approvals, unlocks, manual grants, revocations, updates, and deletions.
- **Error Traceability**: Warnings and error traces for 4xx/5xx responses and database operations.

---

## 🔒 Using JWT Authentication in Requests

To call a protected route, include the following HTTP header:

```http
Authorization: Bearer <your_jwt_token>
```
