FROM rust:1.79 as build

WORKDIR /usr/src/ISpeakRust

COPY . .

RUN cargo build --release

CMD ["./target/release/ISpeakRust"]
