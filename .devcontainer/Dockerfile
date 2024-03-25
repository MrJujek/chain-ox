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

RUN apt-get install -y nodejs npm