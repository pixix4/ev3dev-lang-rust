FROM debian:stretch-slim

RUN dpkg --add-architecture armel

RUN apt-get update
RUN sed -i "s#deb http://security.debian.org/debian-security stretch/updates main#deb http://deb.debian.org/debian-security stretch/updates main#g" /etc/apt/sources.list

RUN apt-get --yes install curl cmake pkg-config clang g++ g++-arm-linux-gnueabi crossbuild-essential-armel libssl-dev libssl-dev:armel libclang-dev \
    && rm -rf /var/lib/apt/lists/*


RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH "$PATH:/root/.cargo/bin"

RUN rustup target add armv5te-unknown-linux-musleabi armv5te-unknown-linux-gnueabi

ENV PKG_CONFIG_SYSROOT_DIR /usr/arm-linux-gnueabi/
ENV CC_armv5te_unknown_linux_gnueabi arm-linux-gnueabi-gcc
ENV CXX_armv5te_unknown_linux_gnueabi arm-linux-gnueabi-g++
