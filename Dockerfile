FROM rust:1.91.1-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*
CMD ["tail", "-f", "/dev/null"]
