FROM rust:1.56 as builder

RUN apt update && apt install -y python3-dev nano python3-pip && pip install pytest pytest-benchmark

WORKDIR /host

ENTRYPOINT ["/bin/bash"]