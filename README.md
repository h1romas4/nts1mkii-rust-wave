# nts1mkii-rust-wave

![](https://github.com/h1romas4/nts1mkii-rust-wave/workflows/Build/badge.svg)

![NTS-1](https://raw.githubusercontent.com/h1romas4/nts1mkii-rust-wave/main/docs/images/nts1mkii-rust-wave-01.jpg)

This repository is a Rust ported build of the WAVES(osc) synthesizer for the NTS-1 digital kit mkII included in [logue-sdk](https://github.com/korginc/logue-sdk). If you are interested in creating an NTS-1 Digital Kit mkII sound source in Rust, perhaps this repository may be of some use to you.

## WIP

- [ ] Not all of the original Waves are implemented.
    - [x] `K_SHAPE`, `K_SUB_MIX` is not working properly. The setpoints are not being evaluated correctly.
    - [ ] Does not implement `dither` and `bit_res_recip`.
- [ ] There may be other bugs.

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
# for git
touch gcc-arm-none-eabi/EMPTY
cd ..
```

Build NTS-1 digital kit mkII sound unit:

```bash
rustup target add thumbv7em-none-eabihf
cargo build --release --target=thumbv7em-none-eabihf
```

Distribute:

```bash
cargo xtask dist
```

Transfer `dist/osc_waves.nts1mkiiunit` to NTS-1 digital kit mkII.

Enjoy!

## Full Build (Optional - with louge-sdk bindgen)

Build logue-sdk: `components/logue_bind/dist/libnts1mkii.a`

```bash
# workaround patch
components/logue_bind/script/louge-sdk-remove-inline-patch.sh
# build
pushd components/logue_bind
mkdir build && cd build
cmake ..
make
popd
ls -laF components/logue_bind/dist/libnts1mkii.a
```

Build Rust with bindgen: `WITH_LOGUE_SDK_BINDGEN=true`

```bash
WITH_LOGUE_SDK_BINDGEN=true cargo build --release --target=thumbv7em-none-eabihf
```

## License

BSD-3-Clause License

## Dependencies

Thanks for all the open source.

|Name|Version|License|Note|
|-|-|-|-|
|[logue-sdk](https://github.com/korginc/logue-sdk)|`67ad379`|BSD-3-Clause||

## A note about linkers

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

```bash
$ rustc -V
rustc 1.77.2 (25ef9e3d8 2024-04-09)
```

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

For some reason, it becomes out of bounds. Late binaries are overwritten in the next section.

```asm
000005c0 <osc_white@plt>:
 5c0:   0000            movs    r0, r0
 5c2:   0000            movs    r0, r0
 5c4:   f240 2c1c       movw    ip, #540        ; 0x21c
 5c8:   f2c0 0c00       movt    ip, #0
 5cc:   44fc            add     ip, pc
 5ce:   Address 0x00000000000005ce is out of bounds.
```

Passing the `--long-plt` option to linker fixes it, but not sure if it is correct. This repository uses this approach.

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

## A note about ELF

- Requre ELF header magic: Note that OS/ABI is UNIX - System V (0x00).

```
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              DYN (Shared object file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x0
  Start of program headers:          52 (bytes into file)
  Start of section headers:          17276 (bytes into file)
  Flags:                             0x0
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         5
  Size of section headers:           40 (bytes)
  Number of section headers:         19
  Section header string table index: 18
```

- Section .unit_header: It must be of type `unit_header_t`.

```
Disassembly of section .unit_header

00003fb0 <unit_header>:
    3fb0:	00000198 	muleq	r0, r8, r1
    3fb4:	00000504 	andeq	r0, r0, r4, lsl #10
    3fb8:	00020000 	andeq	r0, r2, r0
    3fbc:	4831524f 	ldmdami	r1!, {r0, r1, r2, r3, r6, r9, ip, lr}
    3fc0:	00050400 	andeq	r0, r5, r0, lsl #8
    3fc4:	00020000 	andeq	r0, r2, r0
    ...snip...
```

- Section `.rel.plt` and `.rel.dyn`: Dynamic link must contain only system calls.

```
Relocation section '.rel.plt' at offset 0x18a0 contains 1 entry:
 Offset     Info    Type            Sym.Value  Sym. Name
000041e4  00000f16 R_ARM_JUMP_SLOT   00000000   osc_white

Relocation section '.rel.dyn' at offset 0x1810 contains 18 entries:
 Offset     Info    Type            Sym.Value  Sym. Name
000041e8  00000e15 R_ARM_GLOB_DAT    00000000   bitres_lut_f
000041ec  00002615 R_ARM_GLOB_DAT    00000000   wavesF
000041f0  00002715 R_ARM_GLOB_DAT    00000000   wavesC
000041f4  00003b15 R_ARM_GLOB_DAT    00000000   tanpi_lut_f
000041f8  00004115 R_ARM_GLOB_DAT    00000000   sqrtm2log_lut_f
000041fc  00004215 R_ARM_GLOB_DAT    00000000   wt_saw_lut_f
00004200  00004e15 R_ARM_GLOB_DAT    00000000   schetzen_lut_f
00004204  00005615 R_ARM_GLOB_DAT    00000000   cubicsat_lut_f
00004208  00005815 R_ARM_GLOB_DAT    00000000   wavesE
0000420c  00007715 R_ARM_GLOB_DAT    00000000   wavesA
00004210  00007c15 R_ARM_GLOB_DAT    00000000   log_lut_f
00004214  00008315 R_ARM_GLOB_DAT    00000000   wt_sine_lut_f
00004218  00008f15 R_ARM_GLOB_DAT    00000000   wavesB
0000421c  00009115 R_ARM_GLOB_DAT    00000000   wavesD
00004220  00009315 R_ARM_GLOB_DAT    00000000   pow2_lut_f
00004224  00009c15 R_ARM_GLOB_DAT    00000000   wt_sqr_lut_f
00004228  00009f15 R_ARM_GLOB_DAT    00000000   midi_to_hz_lut_f
0000422c  0000a215 R_ARM_GLOB_DAT    00000000   wt_par_lut_f
```
