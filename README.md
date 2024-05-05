# nts1mkii-rust-wave

This repository is a Rust ported build of the WAVE(osc) synthesizer for the NTS-1 digital kit mkII included in [logue-sdk](https://github.com/korginc/logue-sdk).

This may be useful if you want to write the sound source for the NTS-1 Digital Kit mkII in Rust.

**WIP: No waveform is output yet**

## Build

```bash
git clone --recursive https://github.com/h1romas4/nts1mkii-rust-wave
cargo build --release
readelf -a target/thumbv7em-none-eabihf/release/nts1mkii-rust-wave > dist/nts1mkii-rust-wave.elf.txt
# NTS-1 digital kit mkII sound unit `nts1mkii-rust-wave.nts1mkiiunit`
cp -p target/thumbv7em-none-eabihf/release/nts1mkii-rust-wave dist/nts1mkii-rust-wave.nts1mkiiunit
```

## License

BSD-3-Clause License

## Dependencies

Thanks for all the open source.

|Name|Version|License|
|-|-|--|
|[logue-sdk](https://github.com/korginc/logue-sdk)|`67ad379`|BSD-3-Clause|
