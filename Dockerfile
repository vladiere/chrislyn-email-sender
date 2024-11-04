####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

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


WORKDIR /my-worker

COPY ./my-worker .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /

# Copy our build
COPY --from=builder /my-worker/target/x86_64-unknown-linux-musl/release/my-worker ./

# Use an unprivileged user.
USER my-worker:my-worker

CMD ["/my-worker/my-worker"]

