This project helps to determine the size of the library in a binary build.

Run `$ cargo build --release --all-targets`, then `target/release/check-size-*`
contains an optimized build without (debug) symbols of the various binary in
`bin/`. From my testing, a minimal hello world Rust binary is ~350 KiB large.
Thus, everything above is the overhead of my library.
