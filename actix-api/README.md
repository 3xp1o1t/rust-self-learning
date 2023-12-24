# Actix-web API con Rust

## Table of Contents

- [Actix-web API con Rust](#actix-web-api-con-rust)
  - [Table of Contents](#table-of-contents)
  - [About](#about)
  - [Getting Started](#getting-started)
    - [Step by Step](#step-by-step)
    - [Installing](#installing)
  - [Usage](#usage)

## About

Proyecto de rust usando Actix Web.

## Getting Started

Actix-web es un framework de desarrollo web pragmático y rapidísimo para Rust.

### Step by Step

1. Require rust y actix-web.
2. Conceptos y ejemplos para desarrollar esta API: [Ejemplos](https://github.com/actix/examples/tree/master)
3. ![Actix-web](https://crates.io/crates/actix-web)
4. lib.rs -> Es archivo es como un libro de indices para los módulos.
5. app_config -> Para definir las rutas de una manera mas estructurada y no en el punto de entrada.

```rust
// init project
cargo init actix-api
cd actix-api

// Create project
cargo add actix-web
cargo add env_logger
cargo add log


```

### Installing

## Usage

```bas
cargo run
```

Visita o usa Postman/insomia/etc:

GET: <http://localhost:8080/api/welcome>
GET: <http://localhost:8080/api/status>
GET: <http://localhost:8080/auth/login>
GET: <http://localhost:8080/auth/signup>
