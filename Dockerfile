FROM rust:1.58.1-slim-bullseye as builder
WORKDIR /usr/src/zero2prod
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/pixel-land-backend /usr/local/bin/pixel-land-backend

#not sure if we need to explicitly set the below command, thanks to fly
EXPOSE 8080/tcp
ENTRYPOINT ["pixel-land-backend"]
