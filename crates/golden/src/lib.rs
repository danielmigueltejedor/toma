//! Golden-image tests for Toma's renderer (test-only crate; the library is empty).
//!
//! `tests/golden.rs` builds small procedural projects (no media files), renders one frame of each
//! through the CPU reference compositor (`toma-render`) and compares it with a committed PNG
//! in `goldens/` using `toma_testkit::golden`. The same scenes are composited on the GPU
//! (`toma-gpu`) when an adapter is available and compared with the CPU result.
//!
//! Regenerate references with `TOMA_BLESS=1 cargo test -p toma-golden`.
