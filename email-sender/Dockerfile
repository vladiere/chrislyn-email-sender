# Use the official Rust image as the builder
FROM rust:1.82 as builder

# Set the working directory
WORKDIR /email-backend-worker

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new empty shell project to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release
RUN rm -f target/release/deps/your_project_name*

# Now copy the actual source code
COPY . .

# Build the actual application
RUN cargo build --release

# Use a smaller base image to reduce the size
FROM debian:buster-slim

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/your_project_name /usr/local/bin/your_project_name

# Expose the port
EXPOSE 3000

# Command to run the application
CMD ["email_backend_worker"]

