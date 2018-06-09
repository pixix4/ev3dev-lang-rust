FROM ubuntu:18.04

RUN apt update

# Install curl for rust installation
# Install g++ as buildscript compiler
# Install g++-arm-linux-gnueabi as cross compiler
RUN apt --yes install curl g++ g++-arm-linux-gnueabi

# Instull rust for host platform
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH "$PATH:/root/.cargo/bin"

# Add stdlib for target platform
RUN rustup target add armv5te-unknown-linux-gnueabi

