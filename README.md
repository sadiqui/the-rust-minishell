<h2 align="center">the-rust-minishell 🦀</h2>

A lightweight terminal shell implementation in Rust with basic Unix-like commands.

## Features

- Basic shell commands (`cd`, `echo`, `pwd`, etc.)
- Colored terminal output
- Simple error handling
- Cross-platform support

## Project Structure

```
minishell/
├── Cargo.lock      # Dependencies versions
├── Cargo.toml      # Rust project config
├── main.rs         # Shell entry point
└── commands/
    └── mod.rs      # Command implementations
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/sadiqui/the-rust-minishell.git
cd the-rust-minishell
```

2. Build and run:
```bash
cargo run
```

## Available Commands

- `cd` - Change directory
- `echo` - Print text
- `pwd` - Show current directory
- `clear` - Clear screen
- `help` - Show available commands
- `exit` - Quit the shell

## Dependencies

- Rust 1.75+
- `colored` (for colored output)
- `dirs` (for home directory detection)

## License

Open-sourced under [The MIT License](https://opensource.org/license/mit).  
