# Quickner Core

This is where the core of the Quickner project is located. The rust code is located in the `src` directory. The `src` directory contains the following:

- `main.rs` - The main entry point of Rust CLI
- `cli.rs` - The CLI interface
- `config.rs` - The configuration file parser and validator
- `models.rs` - The data models used in the project
- `utils.rs` - The utility functions used in the project

## Building

To build the project, you need to have Rust installed. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install). Once you have Rust installed, you can build the project by running the following command:

```bash
cargo build
```

## Running

```bash
cargo run --release -- -c config.yaml
```

## License

This project is licensed under the Mozilla Public License 2.0. See the [LICENSE](LICENSE) file for details.
