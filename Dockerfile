FROM rust:1.91.1-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*
# coverage pour les tests
RUN rustup component add llvm-tools-preview
RUN cargo install cargo-llvm-cov
CMD ["tail", "-f", "/dev/null"]
