# Compile
FROM    registry.cn-shenzhen.aliyuncs.com/evebattery/rust:latest AS compiler
WORKDIR /
COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/bin/cmpp.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/bin/cmpp.rs#' Cargo.toml
COPY . .
RUN cargo build --release

# Run
FROM    registry.cn-shenzhen.aliyuncs.com/evebattery/golang:ubuntu
# and it's easy to find.
COPY    --from=compiler /target/release/cmpp /app/cmpp
EXPOSE  8888/tcp

WORKDIR /app
CMD   ["./cmpp"]