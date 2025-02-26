## Overview
This is an **intermediate-level** Task Management API built using **Rust** and **Actix Web**. It provides endpoints to create, read, update, and delete tasks.

## Features
- ✅ Create a new task
- ✅ Retrieve all tasks
- Workin on Get a task by ID
- Workin on Update a task
- Workin on Delete a task

## Tech Stack
- **Rust** with **Actix Web** for the backend
- **SQLite** with **SQLx** for database operations
- **Serde** for JSON serialization
- **Tokio** for async runtime
- **UUID** for unique task IDs
- **Dotenv** for environment variables

---

## 🛠️ Setup & Installation

### 1 Prerequisites
- Install **Rust & Cargo**: [Rust Installation](https://www.rust-lang.org/tools/install)
- Install SQLite (or use the SQLite DLL on Windows)

### 2  Clone the Repository
```sh
git clone <repository-url>
cd task_manager
```

### 3 Install Dependencies
```sh
cargo build
```

### 4 Set Up Database
1. Create a `.env` file:
   ```sh
   touch .env
   ```
   Add the following inside `.env`:
   ```
   DATABASE_URL=sqlite://tasks.db
   ```

2. Install `sqlx-cli` (for migrations):
   ```sh
   cargo install sqlx-cli
   ```

3. Create the database and run migrations:
   ```sh
   sqlx database create
   sqlx migrate add create_tasks_table
   ```
   Inside the generated migration file (`migrations/`), add:
   ```sql
   CREATE TABLE IF NOT EXISTS tasks (
       id TEXT PRIMARY KEY,
       title TEXT NOT NULL,
       description TEXT NOT NULL,
       completed BOOLEAN NOT NULL DEFAULT 0
   );
   ```
   Run the migration:
   ```sh
   sqlx migrate run
   ```

---

## 🚀 Running the Server
```sh
cargo run
```
The server will start at `http://127.0.0.1:8080`.

---

## 📌 API Endpoints

### 📝 Create a Task
```sh
curl -X POST http://127.0.0.1:8080/tasks \
     -H "Content-Type: application/json" \
     -d '{"title": "Learn Rust", "description": "Understand Actix Web"}'
```
✅ **Response:**
```json
{
    "success": true,
    "message": "Task created successfully",
    "task_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### 📜 Get All Tasks
```sh
curl -X GET http://127.0.0.1:8080/tasks
```
✅ **Response:**
```json
[
    {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "title": "Learn Rust",
        "description": "Understand Actix Web",
        "completed": false
    }
]
```

### 🔍 Get Task by ID
```sh
curl -X GET http://127.0.0.1:8080/tasks/{task_id}
```

### ✏️ Update a Task
```sh
curl -X PUT http://127.0.0.1:8080/tasks/{task_id} \
     -H "Content-Type: application/json" \
     -d '{"title": "Updated Title", "description": "Updated Description", "completed": true}'
```

### ❌ Delete a Task
```sh
curl -X DELETE http://127.0.0.1:8080/tasks/{task_id}
```

---

## 🛠️ Future Improvements
- ✅ Add authentication (JWT)
- ✅ Implement user roles
- ✅ Add pagination
- ✅ Deploy to a cloud service (AWS, DigitalOcean, etc.)

---

## 📜 License
This project is licensed under the **MIT License**.

---

## 🤝 Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature-branch`)
3. Commit changes (`git commit -m 'Add new feature'`)
4. Push to branch (`git push origin feature-branch`)
5. Open a pull request 🚀

---

## 🔗 References
- [Actix Web Docs](https://actix.rs/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [Tokio Documentation](https://tokio.rs/)
