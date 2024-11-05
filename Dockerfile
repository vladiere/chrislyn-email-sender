####################################################################################################
## Builder
####################################################################################################
FROM rust:nightly AS builder

# Set up the musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=my_worker
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Set up working directory and build the app
WORKDIR /app

# Copy source code
COPY ./my-worker .

# Build the application in release mode for the musl target
RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import the user and group info
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Set up the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/my-worker /app/my-worker

# Switch to the non-root user
USER my_worker:my_worker

# Run the binary
CMD ["/app/my-worker"]

