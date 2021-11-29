Start service:

```sh
docker build -t bionf:latest .
docker run -it --rm -v $(pwd)/src:/host bionf:latest
```

Docker:

```sh
cd rust/
cargo build --release
cp ./target/release/libbioinf02lib.so ../python/myrustlib.so
cd ../python
python3 app.py # or pytest app.py
```
