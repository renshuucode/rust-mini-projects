# Rust Mini Projects

A collection of small, focused Rust projects and libraries built in a workspace (monorepo) style.  
Each `crate` is an independent experiment or utility, leveraging Rust's standard library (`std::fs`, `std::net`, etc.), CLI frameworks, and sharing common utilities where appropriate.

## Features

- 🚀 **Workspace/Monorepo:** All projects are managed in a single repository via Cargo workspaces.
- 🛠️ **Reusable Components:** Common utilities are shared through internal crates.
- 🧪 **Showcase/Practice:** Ideal for learning, rapid prototyping, or demonstrating Rust idioms.

## Project Structure

```
rust-mini-projects/
├── Cargo.toml        # Root workspace config
├── README.md
├── fs-demo/          # Filesystem API explorations
│   └── src/main.rs
├── net-demo/         # Networking experiments
│   └── src/main.rs
├── common-utils/     # Shared helpers/utilities (optional)
│   └── src/lib.rs
└── cli-supertool/    # Example: CLI mini-tool
    └── src/main.rs
```

Each subdirectory (except the root) is a standalone crate (library or binary).

## Getting Started

### Build all projects

```sh
cargo build
```

### Run a specific project

```sh
cargo run -p fs-demo
cargo run -p net-demo
```

### Add a new project

```sh
cargo new crate-name --bin
# Then add the new crate to the [workspace] members in the root Cargo.toml
```

## Contributing

Feel free to fork or submit pull requests if you'd like to add more experiments, tools, or mini-libraries!

## License

MIT

---

Happy hacking and Rust learning!
