FROM rust AS builder

WORKDIR /build
COPY . .

RUN cargo build --release

FROM debian:stable-slim

WORKDIR /

COPY --from=builder /build/target/release/server /usr/bin/server

EXPOSE 8080

CMD ["/usr/bin/server"]
