FROM rust:latest
WORKDIR /rust_project
COPY . .
RUN cargo build
CMD ["cargo", "run"]
