//! OS-specific extensions.
//!
//! These are not cross-platform and their usage should be `cfg`-wrapped.

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
    }
}