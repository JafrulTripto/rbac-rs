# Permissions-rs

A pluggable RBAC (Role-Based Access Control) system built in Rust using SQLx.

## Features
- Assign roles to users.
- Assign permissions to roles.
- Check if a user has a specific role or permission.
- Pluggable implementation with support for both UUID and incremental IDs.

## Table of Contents
- [Getting Started](#getting-started)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

## Getting Started

This crate provides a simple, flexible RBAC system that integrates with a PostgreSQL database. It is designed to be easily integrated into existing applications.

### Prerequisites
- Rust (edition 2021 or later)
- PostgreSQL database

### Setting up the Database
Run the following SQL commands to set up the required tables:

```sql
CREATE TABLE roles (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE permissions (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE user_roles (
    user_id UUID NOT NULL,
    role_id UUID NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

CREATE TABLE role_permissions (
    role_id UUID NOT NULL,
    permission_id UUID NOT NULL,
    PRIMARY KEY (role_id, permission_id),
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);
```

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
permissions-rs = { git = "https://github.com/JafrulTripto/permissions-rs.git" }
```

## Usage

### Initialize the Repository

```rust
use permissions_rs::repository::RbacRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = RbacRepository::new("postgres://username:password@localhost/db_name").await?;
    Ok(())
}
```

### Assign Roles and Permissions

```rust
use permissions_rs::service::RbacService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = RbacService::new("postgres://username:password@localhost/db_name").await?;
    
    // Assign a role to a user
    service.assign_role(user_id, role_id).await?;

    // Check if a user has a specific permission
    let has_permission = service.check_permission(user_id, "read_access").await?;
    println!("User has permission: {}", has_permission);

    Ok(())
}
```

### UUID vs Incremental ID

This library supports both UUID and incremental IDs for entity identifiers. The identifier type can be switched by enabling the appropriate feature in your `Cargo.toml` file:

```toml
[features]
default = ["uuid-id"]
incremental-id = []
uuid-id = []
```

## Configuration

Set the database connection URL and enable the desired features in your `Cargo.toml` file. For example:

```toml
[dependencies]
permissions-rs = { git = "https://github.com/JafrulTripto/permissions-rs.git" }

[features]
default = ["uuid-id"]
uuid-id = []
incremental-id = []
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
