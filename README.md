# Rust SQLx DAO Example

## Overview
Demonstrates implementation of asynchronous database operations using SQLx with a PostgreSQL database.

## Features
- **Async/Await**: Uses Rust's asynchronous programming features to handle database operations.
- **SQLx**: Leverages SQLx for safe and efficient database access.
- **Environment Management**: Uses `dotenv` to manage environment variables.


- **DAO Pattern**: Arguably tries to implement a DAO trait to abstract the database operations.

## Prerequisites
- Rust programming environment (cargo, rustc).
- PostgreSQL server running and accessible.
- `DATABASE_URL` set in your environment or a `.env` file.

## Setup
1. Ensure your PostgreSQL database is accessible and the `DATABASE_URL` environment variable is set correctly.
2. Clone this repository.

## Usage
To run the project:
```bash
cargo run
