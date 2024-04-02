This project helps me to determine the size of the library in an optimized build.
If I build it with `cargo build --release --all-targets`, `target/release/check-size` contains an
optimized build without (debug) symbols of an minimal binary. In my testing, a minimal hello world
Rust binary is ~350kib big. Thus, everything above that is the overhead of my library.

The size of the binary helps to estimate the impact on binaries.
