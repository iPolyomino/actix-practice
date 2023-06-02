# development
FROM rust as develop-stage
WORKDIR /app
RUN cargo install cargo-watch
COPY . .

# build
FROM develop-stage as build-stage
RUN cargo build --release

# release
FROM rust:slim-bullseye
COPY --from=build-stage /app/target/release/api .
EXPOSE 8888
CMD ["/usr/local/bin/api"]
