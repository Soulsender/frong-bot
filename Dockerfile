# Use a Rust base image with Cargo installed
FROM rust:bullseye AS builder

# Set the working directory inside the container
WORKDIR /app

# Now copy the source code
COPY . .

# Build 
RUN cargo update
RUN cargo build --release

# Start a new stage to create a smaller image without unnecessary build dependencies
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/frong-bot-rust .

ENV TOKEN=yourtokenhere
ENV OPENAI_KEY=yourtokenhere

# Command to run the application
CMD ["./frong-bot-rust"]