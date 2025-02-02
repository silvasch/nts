FROM rust:1.77 AS builder
WORKDIR /usr/src/nts
COPY . .
RUN cargo install --path ./crates/nts

FROM debian:bookworm-slim
RUN apt update
COPY --from=builder /usr/local/cargo/bin/nts /usr/local/bin/nts

ENV XDG_CONFIG_HOME=/config
CMD ["nts"]
