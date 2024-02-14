# Rust Raw Bindings for Binaryen C API
[![Binaryen_capi at crates.io](https://img.shields.io/crates/v/crate-name.svg)](https://crates.io/crates/Binaryen_capi-sys)
[![Binaryen_capi at docs.rs](https://docs.rs/crate-name/badge.svg)](https://docs.rs/Binaryen_capi-sys)

## Requestments
* Installed Binaryen  (If it is nothing, this library will build binaryen. but need very long time(about 4m30s))


When searching for the binaryen path, this library calculates backwards from the `wasm-opt` path.
Be sure to set binaryen in the Path environment variable.
It can also be specified by setting the `BINARYEN_PATH` environment variable.

## Path Example
```
/BINARYEN_PATH  
 |- lib  
    |- libbinaryen.a[.so]  
 |- include  
    |- binaryen-c.h  
    |- wasm-delegations.def  
```