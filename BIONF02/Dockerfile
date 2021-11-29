FROM rust:1.56 as builder

RUN apt update && apt install -y python3 nano

WORKDIR /python_lib

COPY . ./

RUN cargo build --release

ENTRYPOINT ["/bin/bash"]

# FROM scratch
# COPY --from=builder /python_lib/target/release/libbioinf02lib.so ./
# ENTRYPOINT [ "libbioinf02lib.so" ]
