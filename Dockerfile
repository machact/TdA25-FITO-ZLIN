FROM messense/rust-musl-cross:x86_64-musl as builder
#ENV SQLX_OFFLINE=true
WORKDIR /app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/TdA25-FITO-ZLIN /TdA25-FITO-ZLIN
COPY --from=builder /app/static /static

ENTRYPOINT ["/TdA25-FITO-ZLIN"]

ENV RUST_LOG="info"
ENV PORT="80"

EXPOSE 80