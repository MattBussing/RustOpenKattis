FROM rust:1.63

WORKDIR /usr/src/myapp
COPY . .

RUN rustup component add rustfmt
