# Rust REST API con Axum y MySQL

## Indice

- [Rust REST API con Axum y MySQL](#rust-rest-api-con-axum-y-mysql)
  - [Indice](#indice)
  - [Acerca de](#acerca-de)
  - [Dependencias](#dependencias)
  - [Setup](#setup)
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

## Referencias

[Guía completa](https://medium.com/@raditzlawliet/build-crud-rest-api-with-rust-and-mysql-using-axum-sqlx-d7e50b3cd130)
