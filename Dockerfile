FROM rust:1.67

RUN mkdir -p /app
COPY . /app
WORKDIR /app/target/release

CMD ["frong-bot-rust"]
