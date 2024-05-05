# nts1mkii-rust-wave

![](https://github.com/h1romas4/nts1mkii-rust-wave/workflows/Build/badge.svg)

![NTS-1](https://raw.githubusercontent.com/h1romas4/nts1mkii-rust-wave/main/docs/images/nts1mkii-rust-wave-01.jpg)

This repository is a Rust ported build of the WAVE(osc) synthesizer for the NTS-1 digital kit mkII included in [logue-sdk](https://github.com/korginc/logue-sdk). This may be useful if you want to write the sound unit for the NTS-1 Digital Kit mkII in Rust.

**WIP: No waveform is output yet**

## Build

Clone repository:

```bash
git clone --recursive https://github.com/h1romas4/nts1mkii-rust-wave
```

Install Arm toolchaine (for Linux) - [For other OS](https://github.com/korginc/logue-sdk/tree/master/tools/gcc)

```bash
cd toolchain
rm -Rf gcc-arm-none-eabi/
wget https://developer.arm.com/-/media/Files/downloads/gnu-rm/10.3-2021.10/gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2
tar xvf gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2
rm gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2
mv gcc-arm-none-eabi-10.3-2021.10 gcc-arm-none-eabi
touch gcc-arm-none-eabi/EMPTY # for git
cd ..
```

Build NTS-1 digital kit mkII sound unit:

```bash
cargo build --release
cp -p target/thumbv7em-none-eabihf/release/nts1mkii-rust-wave dist/nts1mkii-rust-wave.nts1mkiiunit
```

Transfer `dist/nts1mkii-rust-wave.nts1mkiiunit` to NTS-1 digital kit mkII.

Enjoy!

## Full Build (Optional - with louge-sdk bindgen)

Build logue-sdk: `dist/libnts1mkii.a`

```bash
mkdir build && cd build
cmake ..
make
cd ..
ls -laF dist/libnts1mkii.a
```

Build Rust with bindgen: `WITH_LOGUE_SDK_BINDGEN=true`

```bash
WITH_LOGUE_SDK_BINDGEN=true cargo build --release
readelf -a target/thumbv7em-none-eabihf/release/nts1mkii-rust-wave > dist/nts1mkii-rust-wave.elf.txt
cp -p target/thumbv7em-none-eabihf/release/nts1mkii-rust-wave dist/nts1mkii-rust-wave.nts1mkiiunit
```

## License

BSD-3-Clause License

## Dependencies

Thanks for all the open source.

|Name|Version|License|
|-|-|--|
|[logue-sdk](https://github.com/korginc/logue-sdk)|`67ad379`|BSD-3-Clause|
