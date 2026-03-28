## gdstk_parse_rs

gdstk_parse_rs is a rust library for parsing GDSII and OASIS files.
This project uses [gdstk](https://github.com/heitzmann/gdstk), which is licensed under the Boost Software License 1.0.

## Installation

```sh
git clone https://github.com/thayashi-tech/gdstk-parse_rs.git
cd gdstk-parse_rs
git clone https://github.com/heitzmann/gdstk.git
sudo apt-get install libclang-dev clang libqhull-dev
cargo build
```

## Examples

create test data by gdstk python binding

```sh
cd test
sh make.sh
cd -
cargo run --example oasis_to_image  test/output/transformation.oas
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
