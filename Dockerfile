FROM rust:1.61

WORKDIR /usr/src/myapp
COPY . .

RUN rustup component add rustfmt
