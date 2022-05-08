FROM rust

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app
COPY . /usr/src/app

RUN cargo build --release

EXPOSE 8888
CMD ["./target/release/actix-practice"]
