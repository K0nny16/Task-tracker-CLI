# Task Tracker CLI

A simple command-line task manager built in Rust.

## Features

* Add tasks
* List tasks (completed and incomplete)
* Mark tasks as done
* Delete tasks
* Persistent storage using JSON

## Usage

### Add a task

```bash
note -a "Buy milk"
```

### Add a task with assignee

```bash
note -a "Buy milk" -p "Karl"
```

### List tasks

```bash
note -l
```

### Mark task as done

```bash
note -d 1
```

### Delete task

```bash
note --delete 1
```

## Tech Stack

* Rust
* clap (CLI argument parsing)
* serde / serde_json (data persistence)

## What I Learned

* Structuring a Rust project using `lib.rs` and `main.rs`
* Working with enums and pattern matching for command handling
* Error handling using `Result` and `Option`
* File I/O and JSON serialization in Rust
* Designing a CLI interface with `clap`
* Managing ownership and mutable references in real use cases

## Design Notes

The application uses a clean separation between:

* CLI parsing (`clap`)
* Command handling (enum + match)
* Business logic (task handlers)
* Storage layer (file persistence)

This keeps the code modular and easy to extend.

## Future Improvements

* Task filtering (completed / pending)
* Better error messages
* Unit and integration tests
* Task editing support
* Improved CLI UX

---

This project was built as part of learning Rust through hands-on projects, focusing on practical usage rather than theory.
