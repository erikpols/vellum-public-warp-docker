# Tells docker to use the latest Rust official image
FROM rust:latest as builder

# Copy our current working directory into the container
WORKDIR /geodata
COPY ./ ./

ENV SQLX_OFFLINE=true

RUN apt-get update && apt-get install -y cmake

# Create the release build
# RUN cargo install --path ./warp
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y cmake
#COPY --from=builder /usr/local/cargo/bin/geodata /usr/local/bin/geodata
COPY --from=builder /geodata/target/release/geodata .

# Generate our self signed certs (change these parameters accordingly!)
# RUN openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.rsa -out cert.pem \
#    -subj "/C=GB/ST=London/L=London/O=Global Security/OU=IT Department/CN=example.com"

# Expose the port our app is running on
ENV PORT=80

EXPOSE 80

# Run the application!
#CMD ["./target/release/geodata"]

CMD ["./geodata"]