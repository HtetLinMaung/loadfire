# Start from the official Rust image
FROM rust:slim-buster

# Install OpenSSL development packages
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Create a new directory for your application
WORKDIR /usr/src/loadfire

# Copy your source code into the Docker image
COPY . .

# Build your application in release mode
RUN cargo build --release

# The final command or entry point that runs your application
CMD ["./target/release/loadfire"]
