//! The `posish` module contains code specific to the Unix-like platforms
//! supported by the `posish` crate.

pub(crate) mod fs;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod darwin;
#[cfg(target_os = "linux")]
mod linux;
