FROM ubuntu:latest

RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"

# Install curl
RUN apt-get update && apt-get install -y curl

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install uzip
RUN apt-get update && apt-get install -y unzip

# Install bun
RUN curl -fsSL https://bun.sh/install | bash

RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

RUN sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev libssl-dev

RUN avm install latest

RUN avm use latest

# Install git
RUN apt-get update && apt-get install -y git
