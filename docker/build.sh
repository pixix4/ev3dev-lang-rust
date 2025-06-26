#!/usr/bin/env bash 

docker buildx build --platform linux/amd64,linux/arm64 -t pixix4/ev3dev-rust:latest .