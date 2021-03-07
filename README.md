# `bela-sys`

Crude bindings for the [Bela](https://bela.io) platform, to cross-compile from a
Linux or OSX host.

# Setup

Install the right tool chain for the Beaglebone black:

```sh
rustup target add armv7-unknown-linux-gnueabihf
rustup toolchain install stable-armv7-unknown-linux-gnueabihf
```

With a bela board plugged in and accessible at `bela.local`, run:

```sh
source bela_setup_local.sh
```

This downloads the right linker, pulls in some required files from the board,
and sets up the `$PATH` environment variable. This MUST be called in each
terminal session that will be used to call `cargo`, but will only download the
files once. The bela board only needs to be plugged in the first time.

# Testing

This should output a sine wave at 440Hz:

```
cargo build --target=armv7-unknown-linux-gnueabihf --example tone
scp target/armv7-unknown-linux-gnueabihf/debug/examples/tone root@bela.local:~
ssh root@bela.local
# ...
# on the bela board
./tone
```

# Licence

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

