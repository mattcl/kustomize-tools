FROM rust:1.72-alpine as builder

RUN apk add musl-dev

WORKDIR /usr/src/kustomize-tools
COPY . .

RUN cargo install --locked --target-dir /target --path .

FROM alpine:3.18
COPY --from=builder /usr/local/cargo/bin/kustomize-tools /usr/local/bin/kustomize-tools
ENTRYPOINT ["kustomize-tools"]
