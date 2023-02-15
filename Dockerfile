FROM rust:1.61.0

WORKDIR /usr/src/trust
COPY . .

RUN cargo install --path .

CMD ["trust"]