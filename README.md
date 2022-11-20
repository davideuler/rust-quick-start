## Run the main.rs

cargo run

## Build and run other program

rustc src/result_test.rs

./result_test

## Run rust jupyter notebooks

```
docker run --name jupyter-rust -d -p 8899:8899 -v `pwd`:/opt/notebooks --platform=linux/amd64 davideuler/jupyter-rust:1.0
docker logs jupyter-rust
```

Then you can access the URL for jupyter with rust support as prompted by the running log of the application:

http://localhost:8899/lab?token=xxxxxxxx

