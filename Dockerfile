# Start from the official Rust image
FROM rust:latest

# Create a new directory in the container
WORKDIR /usr/src/rust_nuggets

# Copy the Cargo.toml and Cargo.lock files to the new directory
COPY Cargo.toml Cargo.lock ./

# This build step will cache your dependencies
RUN apt update && apt install -y libssl-dev pkg-config clang llvm lld
# RUN cargo build --release
# RUN rm src/*.rs

# Copy the source code into the container
COPY src ./src

# Build the application
RUN cargo build --release

# Run the tests
RUN cargo test --release

# Set the start command to run your binary
CMD ["/usr/src/rust_nuggets/target/release/rust_nuggets"]
