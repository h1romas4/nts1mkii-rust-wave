# nts1mkii-rust-wave

![](https://github.com/h1romas4/nts1mkii-rust-wave/workflows/Build/badge.svg)

![NTS-1](https://raw.githubusercontent.com/h1romas4/nts1mkii-rust-wave/main/docs/images/nts1mkii-rust-wave-01.jpg)

This repository is a Rust ported build of the WAVE(osc) synthesizer for the NTS-1 digital kit mkII included in [logue-sdk](https://github.com/korginc/logue-sdk). This may be useful if you want to write the sound unit for the NTS-1 Digital Kit mkII in Rust.

## WIP: This repository is not yet working!

Calling system interface functions in the machine via PLT freezes. ex. `osc_white()`

```rust
#[no_mangle]
pub extern "C" fn unit_note_on(_arg1: u8, _arg2: u8) {
    // TODO: The problem is that it freezes.
    // Relocation section '.rel.plt' at offset 0x5cc contains 1 entry:
    // Offset     Info    Type            Sym.Value  Sym. Name
    // 00000854  00001316 R_ARM_JUMP_SLOT   00000000   osc_white
    unsafe { osc_white() };
}
```

### Rust & lld ver

Does not work.

Probably the PLT code freezes because it moves the pc to the address of the function in the Thumb code it is calling while in Arm mode.

```asm
000005cc <.rel.plt>:
 5cc:	00000854 	andeq	r0, r0, r4, asr r8       ; 0x854 Indicates the address of the GOT.
 5d0:	00001316 	andeq	r1, r0, r6, lsl r3

00000600 <__ThumbV7PILongThunk_osc_white>:
 ;; Thumb mode
 600:   f240 0c24       movw    ip, #36 ; 0x24
 604:   f2c0 0c00       movt    ip, #0
 608:   44fc            add     ip, pc               ; 0x608 + 4 = 0x60c
 ;; to Arm mode
 60a:   4760            bx      ip                   ; 0x24 + 0x60c = 0x630 -->

00000610 <.plt>:
 ;; snip..
 ;;
 ;; 0x630 <--
 ;; Arm mode
 630:	e28fc600 	add	ip, pc, #0, 12               ; 0x630 + 8 = 0x638
 634:	e28cca00 	add	ip, ip, #0, 20
 ;; Is the pc moving to Thumb code while in Arm mode?
 638:	e5bcf21c 	ldr	pc, [ip, #540]!	; 0x21c      ; 0x638 + 540 = 0x854
 63c:	d4d4d4d4 	ldrble	sp, [r4], #1236	; 0x4d4

Disassembly of section .got:

00000848 <.got>:
	...
 ;; Perhaps this is where the elf loader writes the function address
 854:	00000610 	andeq	r0, r0, r0, lsl r6
```

Would a merge of `config->armThumbPLTs` fix this?

- [[lld] Support thumb PLTs #86223](https://github.com/llvm/llvm-project/pull/86223)

### Rust & arm-none-eabi-ld ver

Does not work.

For some reason, it becomes out of bounds. Late binaries are overwritten in the next section. Passing the `--long-plt` option to linker fixes it, but not sure if it is correct.

```asm
000005c0 <osc_white@plt>:
 5c0:   0000            movs    r0, r0
 5c2:   0000            movs    r0, r0
 5c4:   f240 2c1c       movw    ip, #540        ; 0x21c
 5c8:   f2c0 0c00       movt    ip, #0
 5cc:   44fc            add     ip, pc
 5ce:   Address 0x00000000000005ce is out of bounds.
```

Also, if the `0x7`th byte of the ELF header is not 0x0 (OS/ABI: UNIX - System V) instead of 0x61, KORG KONTROL will not transfer it as ELF invalid.

```
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 61 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            ARM
```

### Original louge-sdk gcc & arm-none-eabi-ld ver

Good work.

```
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
```

```asm
000011e0 <osc_white@plt>:
    11e0:	f240 4c28 	movw	ip, #1064	; 0x428
    11e4:	f2c0 0c00 	movt	ip, #0
    11e8:	44fc      	add	ip, pc
    11ea:	f8dc f000 	ldr.w	pc, [ip]
    11ee:	e7fc      	b.n	11ea <osc_white@plt+0xa>
```

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
rustup target add thumbv7em-none-eabihf
cargo build --release
cp -p target/thumbv7em-none-eabihf/release/nts1mkii-rust-wave dist/nts1mkii-rust-wave.nts1mkiiunit
# patch elf header
printf '\x00' | dd of=dist/nts1mkii-rust-wave.nts1mkiiunit bs=1 seek=7 count=1 conv=notrunc
```

Transfer `dist/nts1mkii-rust-wave.nts1mkiiunit` to NTS-1 digital kit mkII.

Enjoy!

## Full Build (Optional - with louge-sdk bindgen)

Build logue-sdk: `dist/libnts1mkii.a`

```bash
# workaround patch
script/louge-sdk-remove-inline-patch.sh
# build
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
# patch elf header
printf '\x00' | dd of=dist/nts1mkii-rust-wave.nts1mkiiunit bs=1 seek=7 count=1 conv=notrunc
```

## License

BSD-3-Clause License

## Dependencies

Thanks for all the open source.

|Name|Version|License|
|-|-|--|
|[logue-sdk](https://github.com/korginc/logue-sdk)|`67ad379`|BSD-3-Clause|
