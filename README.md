# Rust Nuggets

This is a Rust project that includes several binary applications and their corresponding tests. The aim of this Project is simple: Explore rust by feature.

## Project Structure

The project has the following structure:

`.cargo/`: Contains the `config.toml` file for Cargo configuration.<br>
`src/bin/`: Contains the source code for the binary applications.<br>
`src/bin/tests/`: Contains the unit tests for the binary applications.<br>
`Dockerfile`: Defines the Docker image for the application.<br>
`rust-toolchain.toml`: Specifies the Rust toolchain to be used.

Running the Binaries
You can run an application with the cargo run command. For example, to run the blocking_get_request application, use the following command:

```bash
cargo run --bin blocking_get_request
```

Running the Tests
You can run the tests with the cargo test command:

```bash
cargo t
```

Docker
You can pull the Docker image for the application with the docker pull command:

```bash
docker pull 860x9/rust_nuggets:latest
```

Contributing
Contributions are welcome! Please feel free to submit a pull request.

License
This project is licensed under the terms of the MIT license.
