####################################################################################################
## Builder
####################################################################################################
FROM rust:nightly AS builder

# Set up the musl target for static linking and install dependencies in one step
RUN rustup target add x86_64-unknown-linux-musl && \
  apt update && apt install -y musl-tools musl-dev && \
  update-ca-certificates

# Ensure we're using the nightly version of Cargo
RUN rustup default nightly

# Create appuser
ENV USER=cta_worker
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

# Copy the source code into the container
COPY . .

# Build the application in release mode for the musl target
RUN cargo clean && cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Install necessary libraries for running the Rust binary (Musl)
RUN apk add --no-cache libgcc libstdc++ musl

# Import the user and group info
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Set up the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cta-worker /app/cta-worker

# Ensure the binary has the correct permissions
RUN chmod +x /app/cta-worker

# Switch to the non-root user
USER ${USER}:${USER}

# Run the binary
CMD ["/app/cta-worker"]

