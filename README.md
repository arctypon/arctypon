# arctypon

   [![crates.io](https://img.shields.io/crates/v/arctypon.svg)](https://crates.io/crates/arctypon)
   [![core](https://img.shields.io/crates/v/arctypon-core.svg?label=arctypon-core)](https://crates.io/crates/arctypon-core)
   [![docs](https://docs.rs/arctypon-core/badge.svg)](https://docs.rs/arctypon-core)
   [![license](https://img.shields.io/crates/l/arctypon.svg)](LICENSE-MIT)

**Open-source actor-model kernels & runtimes in `no_std` Rust — built on
[Asterinas OSTD/OSDK](https://github.com/asterinas/asterinas).**

> 🚧 Early stage: this repository currently hosts a placeholder crate while
> the kernel is being built.


## The series

| Crate | Purpose | Status |
|---|---|---|
| `arctypon` | Actor kernel built on OSTD | 🚧 early stage |
| `arctypon-core` | Core actor & message abstractions (`no_std`) | 🚧 early stage |
| `arctypon-runtime` | Scheduler and message dispatch | 🚧 early stage |
| `arctypon-macros` | Procedural macros | 🚧 early stage |

## Design principles

- `no_std` first; 100% safe Rust outside the framework layer
- Actors and message passing over shared state
- Small, auditable core

## Building

Kernel development uses the Asterinas OSDK (`cargo-osdk`, x86_64 Ubuntu + QEMU):

    cargo install cargo-osdk
    cargo osdk build
    cargo osdk test

See the [OSDK guide](https://asterinas.github.io/book/osdk/guide/) for prerequisites.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

### Third-party components

arctypon is built on Asterinas OSTD and OSDK (MPL-2.0). Using them as
dependencies does not affect the licensing of arctypon's own source files;
files copied or modified from MPL-2.0 code remain under MPL-2.0, with source
available at <https://github.com/asterinas/asterinas>.

## Trademarks

"arctypon"™ and the arctypon logo are trademarks of 靈月夜 (Luna-night).
The code licenses grant no trademark rights. Forks must be renamed and
clearly marked as unofficial.

arctypon is an independent project and is not affiliated with or endorsed
by the Asterinas project.
