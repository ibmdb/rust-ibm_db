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

MACOS:
If you get an error i.e. "dyld: Library not loaded: libdb2.dylib"
Run the following command(Where replace the <RUST_CRATE_LIB> with the path of your rust program root folder):

```
install_name_tool -change libdb2.dylib $IBM_DB_HOME/lib/libdb2.dylib <RUST_CRATE_LIB>/target/debug/ibm_db

```


Crates.io link: https://crates.io/crates/ibm_db