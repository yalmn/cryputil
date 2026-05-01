FROM rust:1.83-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release -p cryputil-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/cryputil-server /usr/local/bin/cryputil-server
ENV PORT=8080
EXPOSE 8080
CMD ["cryputil-server"]
