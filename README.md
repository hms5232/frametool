# frametool
Wrapper for framework_tool.

## build

* Ensure the `libudev-dev` is installed. If not, run `apt install libudev-dev`.
* Ensure the `pkg-config` is installed. If not, run `apt install pkg-config`.
* Ensure `$PKG_CONFIG_PATH` is set to the directory containing `libudev.pc`. For example:
    ```bash
    export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig/"

* Because of the need for root permissions to access the EC, we need to use `sudo` to run the program. Sometimes the `sudo -E cargo run` may not work, you can run
    
  ```bash
  cargo build && sudo ./target/debug/frametool
  ```

  instead.
