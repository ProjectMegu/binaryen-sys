FROM rust:latest

RUN --mount=type=cache,target=/var/lib/apt/lists \
  --mount=type=cache,target=/var/cache/apt/archives \
  apt-get update && \
  apt-get install -y clang cmake ninja-build