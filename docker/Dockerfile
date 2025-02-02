FROM rust:1.77 AS builder
WORKDIR /usr/src/nts
COPY . .
RUN cargo install --path ./crates/nts
RUN cargo install --path ./crates/nts_set_pwd

FROM debian:bookworm-slim
RUN apt update
COPY --from=builder /usr/local/cargo/bin/nts /usr/local/bin/nts
COPY --from=builder /usr/local/cargo/bin/nts_set_pwd /usr/local/bin/nts_set_pwd

CMD ["nts"]
