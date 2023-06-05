# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the Rust project files to the working directory
COPY . .

# Build the Rust program
# RUN cargo build --release

# Set the command to run the Rust program
CMD ["cargo", "run"]

