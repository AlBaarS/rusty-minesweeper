# Use an official Rust image for building
FROM rust:1.84 AS builder

# Set the working directory
WORKDIR /rusty-minesweeper

# Copy Cargo files and fetch dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy actual source code and build
COPY . .
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bullseye-slim
WORKDIR /rusty-minesweeper

# Copy compiled binary from the builder stage
COPY --from=builder /rusty-minesweeper/target/release/ /rusty-minesweeper/backend

# Expose the required port
EXPOSE 3001

# Run the Rust backend
CMD ["/rusty-minesweeper/backend"]