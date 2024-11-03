FROM messense/rust-musl-cross:x86_64-musl as builder
#ENV SQLX_OFFLINE=true
WORKDIR /app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/TdA25-FITO-ZLIN /TdA25-FITO-ZLIN
ENTRYPOINT ["/TdA25-FITO-ZLIN"]
ENV PORT="80"
EXPOSE 80