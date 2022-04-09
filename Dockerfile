FROM rust:alpine as builder

RUN apk add -U build-base openssl-dev

WORKDIR /usr/src/build
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust:alpine,source=/usr/local/cargo \
    --mount=type=cache,target=/usr/src/build/target \
    cargo build --release --bin create_new_db --bin banoogle && \
    cp /usr/src/build/target/release/create_new_db . &&\
    cp /usr/src/build/target/release/banoogle .

RUN ./create_new_db


FROM scratch

COPY --from=builder /usr/src/build/banoogle /banoogle
COPY --from=builder /usr/src/build/db /db

EXPOSE 3000
ENTRYPOINT ["/banoogle"]
CMD []
