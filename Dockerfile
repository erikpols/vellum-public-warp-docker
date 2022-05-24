# Tells docker to use the latest Rust official image
FROM rust:latest as builder

# Copy our current working directory into the container
WORKDIR /geodata
COPY ./ ./

RUN apt-get update && apt-get install -y cmake

# Create the release build
# RUN cargo install --path ./warp
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y cmake
COPY --from=builder /geodata/target/release/geodata .

# Expose the port our app is running on
ENV PORT=3022
EXPOSE 3022

# Run the application!
CMD ["./geodata"]