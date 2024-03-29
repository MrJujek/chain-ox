FROM ubuntu:latest

ARG SOLANA_CLI="v1.18.2"
ARG ANCHOR_CLI="v0.29.0"
ARG NODE_VERSION="v18.16.0"

ENV HOME="/root"
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV PATH="${HOME}/.local/share/solana/install/active_release/bin:${PATH}"
ENV PATH="${HOME}/.nvm/versions/node/${NODE_VERSION}/bin:${PATH}"

# Install needed packages
RUN apt-get -qq update && apt-get install -y -q \
    build-essential \
    curl \
    git \
    wget \
    pkg-config \
    libudev-dev \
    libssl-dev \
    python3-pip \
    jq

# Install tgh
RUN curl -s https://raw.githubusercontent.com/dkomeza/tiny-git-helper/main/scripts/install.sh | bash

# Install nvm, node, yarn
SHELL ["/bin/bash", "--login", "-i", "-c"]
RUN wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
RUN source /root/.bashrc && nvm install --lts
RUN source /root/.bashrc && npm install -g yarn
SHELL ["/bin/bash", "--login", "-c"]

# Install rust
RUN curl "https://sh.rustup.rs" -sfo rustup.sh && \
    sh rustup.sh -y && \
    rustup component add rustfmt clippy

# Install solana
RUN sh -c "$(curl -sSfL https://release.solana.com/${SOLANA_CLI}/install)"

# Install anchor
RUN cargo install --git https://github.com/coral-xyz/anchor --tag ${ANCHOR_CLI} anchor-cli --locked

# Install avm
RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
RUN avm install latest
RUN avm use latest

# Build a dummy program to bootstrap the BPF SDK (doing this speeds up builds)
RUN mkdir -p /tmp && cd tmp && anchor init dummy && cd dummy && \
    echo 'anchor-spl = "0.29.0"' >> ./programs/dummy/Cargo.toml && \
    echo 'mpl-candy-machine-core = "3.0.1"' >> ./programs/dummy/Cargo.toml && \
    echo 'mpl-candy-guard = "3.0.0"' >> ./programs/dummy/Cargo.toml && \
    cargo build
RUN cd /tmp/dummy && \
    anchor build