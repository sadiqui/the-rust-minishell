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
# or git clone https://learn.zone01oujda.ma/git/asadiqui/0-shell
cd the-rust-minishell
# or cd 0-shell
```

2. Build and run:
```bash
cargo run
```

## Available Commands

- `cd [dir]` - Change directory
- `pwd` - Show current directory
- `cat [file]` - Display file contents
- `cp [src] [dst]` - Copy file from src to dst
- `echo [text]` - Print text
- `clear` - Clear screen
- `help` - Show commands
- `exit` - Exit shell

## Dependencies

- Rust 1.75+
- `colored` (for colored output)
- `ctrlc` (for Ctrl+C handling)
- `dirs` (for home directory detection)

## License

Open-sourced under [The MIT License](https://opensource.org/license/mit).  
