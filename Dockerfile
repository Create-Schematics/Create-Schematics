# Use an official Rust runtime as a parent image
FROM rust:latest

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Build the Rust application
RUN cargo build --release
