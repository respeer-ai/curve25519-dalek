// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2021 isis lovecruft
// Copyright (c) 2016-2019 Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - isis agora lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

#![no_std]
#![cfg_attr(all(curve25519_dalek_backend = "simd", nightly), feature(stdsimd))]
#![cfg_attr(
    all(curve25519_dalek_backend = "simd", nightly),
    feature(avx512_target_feature)
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg, doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]
//------------------------------------------------------------------------
// Documentation:
//------------------------------------------------------------------------
#![doc(
    html_logo_url = "https://cdn.jsdelivr.net/gh/dalek-cryptography/curve25519-dalek/docs/assets/dalek-logo-clear.png"
)]
#![doc = include_str!("../README.md")]
//------------------------------------------------------------------------
// Linting:
//------------------------------------------------------------------------
#![cfg_attr(allow_unused_unsafe, allow(unused_unsafe))]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

// needed for engine25519-as.
#![recursion_limit="512"]

//------------------------------------------------------------------------
// External dependencies:
//------------------------------------------------------------------------

#[cfg(feature = "alloc")]
#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

// TODO: move std-dependent tests to `tests/`
#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(feature = "digest")]
pub use digest;

// Internal macros. Must come first!
#[macro_use]
pub(crate) mod macros;

//To consider upstreaming, we likely can't do this. Consider the "panic_on_sw_eval" feature
#[allow(unused_imports)]
#[cfg(any(test, curve25519_dalek_backend = "u32e_backend"))]
#[macro_use]
extern crate engine25519_as;
#[cfg(curve25519_dalek_backend = "u32e_backend")] //this is the binding for betrusted, so it should be gated 
                                                  //with a "betrusted" flag, but we gate it with the backend 
                                                  //flag for now. We'd need to refactor this to be
                                                  //make it easier to support other platforms,
                                                  //though there are no other platforms. For
                                                  //upstreaming this might be diserable, but for
                                                  //now, we'll leave it as a TODO.
extern crate engine_25519;

#[cfg(curve25519_dalek_backend = "u32e_backend")] //while this is specific to betrusted, any other
                                                  //use of this hardware would likely also need
                                                  //utralib, at least that would be easiest.
extern crate utralib;

//------------------------------------------------------------------------
// curve25519-dalek public modules
//------------------------------------------------------------------------

// Scalar arithmetic mod l = 2^252 + ..., the order of the Ristretto group
pub mod scalar;

// Point operations on the Montgomery form of Curve25519
pub mod montgomery;

// Point operations on the Edwards form of Curve25519
pub mod edwards;

// Group operations on the Ristretto group
pub mod ristretto;

// Useful constants, like the Ed25519 basepoint
pub mod constants;

// External (and internal) traits.
pub mod traits;

//------------------------------------------------------------------------
// curve25519-dalek internal modules
//------------------------------------------------------------------------

// Finite field arithmetic mod p = 2^255 - 19
pub(crate) mod field;

// Arithmetic backends (using u32, u64, etc) live here
#[cfg(docsrs)]
pub mod backend;
#[cfg(not(docsrs))]
pub(crate) mod backend;

// Generic code for window lookups
pub(crate) mod window;

pub use crate::{
    edwards::EdwardsPoint, montgomery::MontgomeryPoint, ristretto::RistrettoPoint, scalar::Scalar,
};

// Build time diagnostics for validation
#[cfg(curve25519_dalek_diagnostics = "build")]
mod diagnostics;
