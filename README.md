## Mahjong seatings Rust library

Small library to calculate seatings for mahjong or any other game for 4 persons.

For WebAssembly sources and builds, see the [repo](https://github.com/MahjongPantheon/mahjong-seatings-rs) and npm
packages for [node](https://npmjs.com/package/mahjong-seatings-rs-node)
and [bundlers](https://npmjs.com/package/mahjong-seatings-rs-bundlers).

### Build

```rust
cargo build --release
```

### Usage

Install the library by adding the following in your Cargo.toml:

```toml
[dependencies]
# write last version from https://github.com/MahjongPantheon/mahjong-seatings-rust/blob/main/Cargo.toml
mahjong-seatings-rust = { git = "https://github.com/MahjongPantheon/mahjong-seatings-rust.git", version = "1.1.0" } 
```

For details about usage, refer to unit tests in corresponding files.

### Credits

Swiss seating algorithm is taken from [mahjongsoft site](http://mahjongsoft.ru/seating.shtml) and ported to several
languages including Rust.
