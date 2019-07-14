FROM rustlang/rust:nightly

RUN cargo install cargo-watch

WORKDIR /app

EXPOSE 8000/tcp

VOLUME ["/usr/local/cargo"]