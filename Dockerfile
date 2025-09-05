# Use a Rust base image with Cargo installed
FROM rust:slim-bullseye AS builder

# Set the working directory inside the container
WORKDIR /app

# Now copy the source code
COPY . .

# Build 
RUN cargo update
RUN cargo build --release && strip target/release/frong-bot-rust

# Start a new stage to create a smaller image without unnecessary build dependencies
FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/frong-bot-rust .
RUN mkdir -p ./assets
COPY ./assets ./assets

ENV TOKEN=yourtokenhere
ENV OPENAI_KEY=yourtokenhere

CMD ["./frong-bot-rust"]
