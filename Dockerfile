FROM rust:1.59

WORKDIR /usr/src/myapp
COPY . .

RUN rustup component add rustfmt
