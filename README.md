# rust-cli-project

A simple Rust CLI todo list application.

## Usage

Build the project:

```powershell
cargo build --release
```

Run the CLI:

```powershell
cargo run -- add "Buy milk"
cargo run -- list
cargo run -- done 1
cargo run -- remove 1
cargo run -- clear
```

This tool stores tasks in a local `todo.json` file in the project directory.
