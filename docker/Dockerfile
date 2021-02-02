FROM debian:stretch

RUN dpkg --add-architecture armel
RUN apt update

# Fix debian package alias
RUN sed -i "s#deb http://security.debian.org/debian-security stretch/updates main#deb http://deb.debian.org/debian-security stretch/updates main#g" /etc/apt/sources.list

# Install curl for rust installation
# Install g++ as buildscript compiler
# Install g++-arm-linux-gnueabi as cross compiler
RUN apt --yes install curl g++ g++-arm-linux-gnueabi crossbuild-essential-armel

# Instull rust for host platform
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH "$PATH:/root/.cargo/bin"

# Add stdlib for target platform
RUN rustup target add armv5te-unknown-linux-gnueabi

# docker build . -t pixix4/ev3dev-rust-cross --no-cache

# docker run -it --rm -v $PWD:/build/ -w /build pixix4/ev3dev-rust-cross
# cargo build --release --target armv5te-unknown-linux-gnueabi
