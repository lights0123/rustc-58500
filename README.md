# Building
Make sure you're running on Rust nightly. Then, install
[cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild).
```bash
rustup component add rust-src
cargo install cargo-xbuild
```
Then, build it:
```bash
cargo xbuild --target armv5te-nspire-eabi.json --release
```

Unless you really *want* to install the
[full toolchain](https://github.com/ndless-nspire/Ndless/wiki/Ndless-SDK:-C-and-assembly-development-introduction),
you will get a linker not found error. However, if you run `strings
target/sysroot/lib/rustlib/armv5te-nspire-eabi/lib/libcore-*.rlib | grep
__sync` at this point, you can see that there are several
`__sync_val_compare_and_swap_x` instructions still there.
