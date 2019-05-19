# Neptune

Neptune is my attempt at writing an Operating System from scratch in Rust.

# Building

1. You'll need nightly version of latest rust compiler. A simple, slightly dangerous way to do this is to run `curl -sSf https://sh.rustup.rs | bash` and follow the instructions.

2. The OS for now uses BIOS to boot. I did not wrote the required ASM to boot into 16 bit and switch from 16 bit -> protected mode for this operating system because I had already done that in an older attempt at writing an operating system.
So, For this project, I used [bootimage](https://crates.io/crates/cargo-bootimage). Install it using `cargo install cargo-bootimage`.

3. Install `xargo` using `cargo install xargo`.

# Running

    cargo xrun --release --target ./neptune-x86_64.json

# Running tests

    cargo xtest --release --target ./neptune-x86_64.json

# Future Plans

1. Boot with UEFI.
2. Implement a short term scheduler.
3. Threads.
4. A Graphics Subsystem, mainly to be used to implement games and not an actual GUI interface.


# License

MIT
