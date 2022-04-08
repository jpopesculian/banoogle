FROM rust:alpine as builder

RUN apk add -U build-base

WORKDIR /usr/src/build
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust:alpine,source=/usr/local/cargo \
    --mount=type=cache,target=/usr/src/build/target \
    cargo build --release --bins && \
    cp /usr/src/build/target/release/banoogle .

FROM scratch

COPY --from=builder /usr/src/build/banoogle .

EXPOSE 3000
ENTRYPOINT ["./banoogle"]
CMD []
