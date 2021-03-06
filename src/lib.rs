//! [LAPACK] source of choice.
//!
//! The usage of the package is explained [here][usage].
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`, which is the one in the [Accelerate] framework (macOS only),
//! * `netlib`, which is the reference one by [Netlib], and
//! * `openblas`, which is the one in [OpenBLAS].
//!
//! An implementation can be chosen as follows:
//!
//! ```toml
//! [dependencies]
//! lapack-src = { version = "0.1", features = ["accelerate"] }
//! lapack-src = { version = "0.1", features = ["netlib"] }
//! lapack-src = { version = "0.1", features = ["openblas"] }
//! ```
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//! [netlib]: http://www.netlib.org/
//! [openblas]: http://www.openblas.net/
//! [usage]: https://blas-lapack-rs.github.io/usage

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;
