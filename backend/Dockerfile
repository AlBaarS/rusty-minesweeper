# Use an official Rust image for building
FROM rust:1.84-bullseye AS builder

# Set the working directory
WORKDIR /rusty-minesweeper

# Copy Cargo files and source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Fetch dependencies separately to optimize caching
RUN cargo build --release

# Copy the rest of the project
COPY . .

# Build the final release binary
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bullseye
WORKDIR /rusty-minesweeper

# Copy compiled binary from the builder stage
COPY --from=builder /rusty-minesweeper/target/release/ /rusty-minesweeper/backend

# Expose the required port
EXPOSE 3001

# Run the Rust backend
CMD ["/rusty-minesweeper/backend/main"]