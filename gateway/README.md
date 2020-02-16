# StandaertHA MQTT Gateway

## Cross-compiling for Raspberry Pi 3 (ARM)

This seems to work alright, regardless of the distro I have on Raspberry Pi.

- Install Rust (with `rustup`)
- Download and extract http://musl.cc/arm-linux-musleabihf-cross.tgz
- Create `.cargo/config` with contents:
```toml
[target.arm-unknown-linux-musleabihf]
linker = "/path/to/arm-linux-musleabihf-cross/bin/arm-linux-musleabihf-gcc"
```
- Add `arm-unknown-linux-musleabihf` target with `rustup`:
```sh
rustup target add arm-unknown-linux-musleabihf
```
- For debug build (note the trailing hyphen in the `CROSS_COMPILE` env variable):
```sh
CROSS_COMPILE=/path/to/arm-linux-musleabihf-cross/bin/arm-linux-musleabihf- cargo build --target=arm-unknown-linux-musleabihf
```
- For release build, add `--release`