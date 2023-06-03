# upload-server

Super simple upload server written in Rust, using [Rocket.rs](https://rocket.rs) and [`rocket-multipart-form-data`](https://crates.io/crates/rocket-multipart-form-data).
Can be used with a frontend like [this one](https://obiw.ac/upload).

## Configuration

You can configure the port in the `Rocket.toml` file:

```toml
port = 6969
```

The default value is `4567`.

## Running

Just do:

```console
cargo run
```
