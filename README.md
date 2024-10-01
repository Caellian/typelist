# Variadic Tuples

Experiments with using tuples in place of vararg generics in Rust.

Maybe it turns into a crate if I get the types to behave as one would expect.
Who knows.

## Alternatives

- [`frunk`](https://crates.io/crates/frunk) - far more developed and functional.
  - Actual list representation is a nested 2-tuple whereas this library uses
    n-tuple representation to be more in line with end-user expectations.
  - This library provides no means of turning structs into type lists.

## License

This repository is licensed under terenary
[MIT](./LICENSE_MIT)/[Apache-2.0](./LICENSE_APACHE)/[Zlib](./LICENSE_ZLIB)
license.
