FROM rust:1.65.0

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["myapp"]