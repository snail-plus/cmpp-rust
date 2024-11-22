# Compile
FROM    registry.cn-shenzhen.aliyuncs.com/evebattery/rust:latest AS compiler
WORKDIR /
COPY    . .
RUN     set -eux; \
        cargo build --release

# Run
FROM    registry.cn-shenzhen.aliyuncs.com/evebattery/alpine:latest
# and it's easy to find.
COPY    --from=compiler /target/release/cmpp /app/cmpp
EXPOSE  8888/tcp

WORKDIR /app
CMD   ["./cmpp"]