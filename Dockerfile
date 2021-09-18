FROM rust:1.40 as builder
WORKDIR /usr/src/myapp
COPY Cargo.toml .
COPY src src
RUN rustup default nightly

COPY benches benches 
RUN cargo install --path .

CMD ["cargo", "bench"]
