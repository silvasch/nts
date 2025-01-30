FROM rust:1.77

WORKDIR /usr/src/nts
COPY . .

RUN cargo install --path .

ENV XDG_CONFIG_HOME=/config
CMD ["nts"]
