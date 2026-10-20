FROM debian:buster-slim

RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
    apt-get install -y \
        curl \
        unzip \
        apt-transport-https \
        ca-certificates \
        software-properties-common \
        sudo \
        supervisor \
        jq \
				git \
				musl-tools \
        iputils-ping \
        build-essential \
        zlib1g-dev \
        gettext \
        liblttng-ust0 \
        libcurl4-openssl-dev \
				libssl-dev \
				pkg-config \
				libssl-dev \
				clang \
				llvm \
				awscli \
				neovim \
				locales \
				locales-all \
        openssh-client && \
    rm -rf /var/lib/apt/lists/* && \
    apt-get clean

# Create user
RUN useradd --create-home --shell /bin/bash runner
WORKDIR /home/runner
USER runner

# musl arm64 request (rustls-tls): https://github.com/briansmith/ring/issues/1414#issuecomment-1055177218
ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/runner/.cargo/bin:${PATH}"

ENV LC_ALL en_US.UTF-8
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US.UTF-8

# Separate target dir, so when mount does not overlap with host - weird issues during build
ENV CARGO_TARGET_DIR=/tmp/target

RUN cargo --help

WORKDIR /home/runner/src
CMD cargo test
