FROM rust:alpine as builder

WORKDIR /usr/src/hello
COPY . .

RUN apk add --no-cache musl-dev
RUN cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/hello       /usr/local/bin/hello
COPY --from=builder /usr/src/hello/*.html            /usr/share/hello/
COPY --from=builder /usr/src/hello/hello-config.toml /etc/

CMD ["hello"]