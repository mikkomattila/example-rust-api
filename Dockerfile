FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /example-api-rust

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /example-api-rust/target/x86_64-unknown-linux-musl/release/example-api-rust /example-api-rust
ENTRYPOINT ["/example-api-rust"]
EXPOSE 3000
