## Run the main.rs

cargo run

## Build and run other program

cargo run --bin result_test
cargo run --bin trait_and_dyn
cargo run --bin borrow_test
cargo run --bin box_and_rc
cargo run --bin struct_impl_and_lambda
cargo run --bin struct_impl_simple



## Run rust jupyter notebooks

```
docker run --name jupyter-rust -d -p 8899:8899 -v `pwd`:/opt/notebooks --platform=linux/amd64 davideuler/jupyter-rust:1.0
docker logs jupyter-rust
```

Then you can access the URL for jupyter with rust support as prompted by the running log of the application:

http://localhost:8899/lab?token=xxxxxxxx

