# Rust todo cli

## Description

Mini project to learn diesel.rs and rust.

Main function is TODO list controlled by terminal.

## Stack

- Rust 1.3
- Diesel.rs
- Docker

## Function

### Requirements:

- diesel_cli

Before execute function, execute this:

```
make start

diesel migration run
```

### Show todo

```bash
cargo run --bin show_todo
```

### Add todo

```bash
cargo run --bin add_task
```

### Remove todo