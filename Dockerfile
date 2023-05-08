FROM rust:1.69-slim as builder

WORKDIR app
COPY . /app

RUN cargo install --path .


# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp


CMD ["myapp"]