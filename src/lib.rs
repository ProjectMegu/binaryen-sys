//! # Rust Raw Bindings for Binaryen C API
//! ## Requestments
//! * Installed Binaryen  (If it is nothing, this library will build binaryen. but need very long time(about 4m30s))
//! * clang (for genetate binaryen)
//! * libstdc++
//! 
//! When searching for the binaryen path, this library calculates backwards from the `wasm-opt` path.
//! Be sure to set binaryen in the Path environment variable.
//! It can also be specified by setting the `BINARYEN_PATH` environment variable.
//! 
//! ## Path Example
//! ```
//! /BINARYEN_PATH  
//!  |- lib  
//!     |- libbinaryen.a[.so]  
//!  |- include  
//!     |- binaryen-c.h  
//!     |- wasm-delegations.def  
//! ```

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
