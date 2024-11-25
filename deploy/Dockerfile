# Compile
FROM registry.cn-shenzhen.aliyuncs.com/evebattery/rust:latest AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# Run
FROM    registry.cn-shenzhen.aliyuncs.com/evebattery/golang:ubuntu
# and it's easy to find.
COPY    --from=builder /app/target/release/cmpp /app/cmpp
EXPOSE  8888/tcp

WORKDIR /app
CMD   ["./cmpp"]