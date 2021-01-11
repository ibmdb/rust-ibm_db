# ibm_db
Considering this is to be used by RUST developer, it is assumed to have RUST language up and running in your system.
Confirm by typing below in command cprompt:
>rustc --version

RUST Driver:
Tested on rustc 1.42.0 and above.

Update CLI driver path i.e. $DRIVER_HOME/lib in [build.rs](build.rs) file in case CLI driver already downloadd/present in system.

If CLI driver not present, kindly use [setup.rs](examples/setup.rs) file to download and configure the same.

```
cargo run --package Rust_C_Sample --example setup

```

Compile & Run src(main.rs) using:

```
cargo run

```


Crates.io link: https://crates.io/crates/ibm_db