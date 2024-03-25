FROM ubuntu:latest

RUN apt-get -qq update

RUN apt-get install -y -q \
    build-essential \
    curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo --version

RUN apt install -y git

RUN apt-get install -y wget

RUN apt-get install -y pkg-config build-essential libudev-dev libssl-dev

RUN wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.0g-2ubuntu4_amd64.deb

RUN dpkg -i libssl1.1_1.1.0g-2ubuntu4_amd64.deb

RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"

RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

RUN avm install latest

RUN avm use latest

RUN curl -s https://raw.githubusercontent.com/dkomeza/tiny-git-helper/main/scripts/install.sh | bash

SHELL ["/bin/bash", "--login", "-i", "-c"]
RUN wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
RUN source /root/.bashrc && nvm install --lts
RUN source /root/.bashrc && npm install -g yarn
SHELL ["/bin/bash", "--login", "-c"]