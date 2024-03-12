# Rust REST API con Axum y MySQL

## Indice

- [Rust REST API con Axum y MySQL](#rust-rest-api-con-axum-y-mysql)
  - [Indice](#indice)
  - [Acerca de](#acerca-de)
  - [Dependencias](#dependencias)
  - [Setup](#setup)
  - [Hot reload para proyecto de rust](#hot-reload-para-proyecto-de-rust)
  - [Referencias](#referencias)

## Acerca de

Proyecto para aprender de SQLx con rust y Axum

`SQLx no tiene soporte para SQL Server :'(`

## Dependencias

Rust crates

```bash
cargo add axum
cargo add tokio -F full
cargo add tower-http -F "cors"
cargo add serde_json
cargo add serde -F derive
cargo add chrono -F serde
cargo add dotenv
cargo add uuid -F "serde v4"
cargo add sqlx --features "runtime-async-std-native-tls mysql chrono uuid"
```

## Setup

Crear volumen para persistencia de datos

```bash
docker volume create mysql-db-data
```

Crear imagen de MySQL

```bash
docker run -d -p 33060:3306 --name mysql-db  -e MYSQL_ROOT_PASSWORD=123456 --mount src=mysql-db-data,dst=/var/lib/mysql mysql
```

Entrar en el contenedor

```bash
docker exec -it mysql-db mysql -p
```

Crear la Base de datos

```bash
# dentro del contenedor de mysql
create database <nombre de tu db en el archivo .env>
```

## Hot reload para proyecto de rust

Normalmente el proyecto se compila y se ejecuta

```bash
cargo build
cargo run
```

Existe una crate llamado `cargo-watch` que funge como hot reloader

Instalar cargo-watch

```bash
cargo install cargo-watch
```

Ejecutar el watch sobre la carpeta src/ que normalmente cambia

```bash
cargo watch -q -c -w src/ -x run
```

Ahora cada cambio en un archivo dentro de src, provocara re-compilar el proyecto y
ejecutarlo por si solo.

## Referencias

[Guía completa](https://medium.com/@raditzlawliet/build-crud-rest-api-with-rust-and-mysql-using-axum-sqlx-d7e50b3cd130)
