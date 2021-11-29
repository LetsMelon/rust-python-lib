Start service:

```sh
/ docker build -t bionf02:latest .
/ docker run -it --rm -v /Users/letsmelon/Desktop/BIOINF/BIONF02/:/host bionf02:latest
```

Docker:

```sh
/       cd host
/host   cargo build --release
/host   cp ./target/release/libbioinf02lib.so ./python/myrustlib.so
/host   cd python
/python python3 app.py # or pytest app.py
```