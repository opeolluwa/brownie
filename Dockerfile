# # the folling listings using docker multibuild concept
# # infor on docker multibuild concept is foun here  https://docs.docker.com/develop/develop-images/multistage-build/
# # infor on dockerizing rust rocket images is found here https://www.koyeb.com/tutorials/deploy-a-rust-web-app-with-rocket and a youtube tutorial here  https://www.youtube.com/watch?v=TDukU7X9WNw 
# # --------------------------------------------------------------------------------

# #Stage One with the Alias "Builder"
# # FROM rust:1 as builder
# FROM ekidd/rust-musl-builder:stable as builder
# WORKDIR /app
# COPY . .
# RUN cargo install --path .
# # RUN cargo build --release

# #Stage Two
# FROM debian:buster-slim as runner
# # FROM scratch
# WORKDIR /app
# # COPY --from=builder /usr/local/cargo/bin/rustly /usr/local/bin/rustly
# COPY --from=builder /usr/local/cargo/bin/rustly ./
# # COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release ./
# COPY --from=builder ./ ./
# #copy the static asssets from the sources
# # COPY --from=builder ./ /app/
# ENV ROCKET_ADDRESS=0.0.0.0
# ENV APP_URL=www.example.com
# EXPOSE 8000
# CMD ["rustly"]



FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin rustly


FROM debian:buster-slim as runner
COPY --from=builder ./target/release/rustly ./
COPY --from=builder ./migrations ./
COPY --from=builder ./public ./
COPY --from=builder ./templates ./
COPY --from=builder ./*.toml ./


ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["rustly"]