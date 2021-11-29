FROM rust:1.56 as builder

RUN apt update && apt install -y python3-dev nano 

RUN mkdir host

ENTRYPOINT ["/bin/bash"]