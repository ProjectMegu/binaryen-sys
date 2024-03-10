FROM rust:latest

RUN --mount=type=cache,target=/var/lib/apt/lists \
  --mount=type=cache,target=/var/cache/apt/archives \
  apt-get update && \
  apt-get install -y clang cmake ninja-build build-essential wget curl git

ARG BINARYEN_VERSION=116
ARG BINARYEN_URL="https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz"
ARG WABT_VERSION=1.0.34
ARG WABT_URL="https://github.com/WebAssembly/wabt/releases/download/${WABT_VERSION}/wabt-${WABT_VERSION}-ubuntu.tar.gz"

RUN mkdir /work
WORKDIR /work

RUN wget -O binaryen.tar.gz ${BINARYEN_URL}
RUN mkdir /root/.binaryen && tar -xzvf binaryen.tar.gz -C /root/.binaryen --strip-components 1
RUN wget -O wabt.tar.gz ${WABT_URL}
RUN mkdir /root/.wabt && tar -xzvf wabt.tar.gz -C /root/.wabt --strip-components 1

WORKDIR /

RUN rm -r -f /work
ENV PATH=/root/.binaryen/bin:/root/.wabt/bin:${PATH}