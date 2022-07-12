# FROM rust:slim-buster as build
FROM ekidd/rust-musl-builder:stable as build
# RUN mkdir /app
WORKDIR /app
COPY . .
RUN cargo clean && cargo build --release
# RUN cargo build --release
# RUN cargo new rust-app
# RUN cd rust-app && cargo build
# RUN cd ..

# RUN ls

# RUN cargo uninstall rustly
# RUN cargo build
# RUN cargo install --path . 

FROM debian:buster-slim

RUN mkdir /app
WORKDIR /app

COPY --from=build /app/target/debug/rustly /app/
# COPY --from=build /app/rustly/target/release/rustly ./app
COPY --from=build /app/migrations /app/migrations
COPY --from=build /app/public /app/public
COPY --from=build /app/templates /app/templates
COPY --from=build /app/*.toml /app/

CMD ["/app/rustly"]
