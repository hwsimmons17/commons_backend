FROM rust:1.67 AS builder
WORKDIR /usr/src/commons_backend
COPY . .
RUN cargo build --release

# FROM alpine:latest  
# WORKDIR /root/
# COPY --from=builder /usr/src/commons_backend/target/release/commons_backend ./
CMD ["./target/release/commons_backend"]