FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .
FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/rustly /usr/local/bin/rustly
ENV ROCKET_ADDRESS=0.0.0.0
ENV APP_URL=www.example.com
EXPOSE 8000
CMD ["rustly"]
