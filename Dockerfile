FROM debian:stretch

ENV DEBIAN_FRONTEND=noninteractive \
  RUSTUP_HOME=/usr/local/rustup \
  CARGO_HOME=/usr/local/cargo \
  PATH=/usr/local/cargo/bin:$PATH

RUN apt-get update && \
  apt-get install -y apt-utils && \
  dpkg-reconfigure apt-utils && \
  apt-get install -y \
    gcc-arm-linux-gnueabihf \
    automake \
    make \
    curl \
    wget \
    g++ \
    git

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --no-modify-path && \
  rustup target add armv7-unknown-linux-gnueabihf

WORKDIR /project
ADD . .

ENTRYPOINT ["cargo", "build", "--release", "--target=armv7-unknown-linux-gnueabihf"]
