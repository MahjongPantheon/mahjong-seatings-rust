## Mahjong seatings Rust library

Small library to calculate seatings for mahjong or any other game for three or four players.

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

### Three-player tables

Use `make_shuffled_seating_with_table_size(..., 3)` for shuffled sanma seating
and `update_wind_placing_with_table_size(..., 3)` for wind placement within
prescripted tables. Inputs contain only real players; every table in the previous
seating history must have exactly three IDs. These entry points also accept `4`;
the original entry points retain their four-player behavior.
`make_interval_seating_with_table_size(..., 3)` keeps the same rating/step grouping
and fallback for remaining tables. `make_swiss_seating_with_table_size(..., 3)`
keeps the same recursive rating-prioritized search and repeated-opponent limits.
Both use three-seat wind placement, including balanced winds.
