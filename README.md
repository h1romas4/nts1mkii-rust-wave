# nts1mkii-rust-wave

![](https://github.com/h1romas4/nts1mkii-rust-wave/workflows/Build/badge.svg)
![](https://github.com/h1romas4/nts1mkii-rust-wave/workflows/Release/badge.svg)

![NTS-1](https://raw.githubusercontent.com/h1romas4/nts1mkii-rust-wave/main/docs/images/nts1mkii-rust-wave-01.jpg)

This repository provides custom oscillators written in Rust for the KORG NTS-1 Digital Kit mkII.

It includes a Rust port of Waves provided by [logue-sdk](https://github.com/korginc/logue-sdk), a 32-byte wavetable synth, and source code templates for the instruments.

If you are interested in creating a sound source for the NTS-1 Digital Kit mkII in Rust, you may find this repository useful.

## Demo

🎥 [Rust build custom oscillator on KORG NTS-1 digital kit mkII](https://www.youtube.com/watch?v=8V_X-8huvJ0)

🎥 [ Rust build table32 custom oscillator on KORG NTS-1 digital kit mkII ](https://www.youtube.com/watch?v=EQm9mchfxXw)

## Release

[Pre-built binaries](https://github.com/h1romas4/nts1mkii-rust-wave/releases/latest)

|Unit name||
|---|---|
|osc_waves.nts1mkiiunit| Rust ports louge-sdk Waves |
|osc_dummy.nts1mkiiunit| osc_sinf for operation check |
|osc_table32.nts1mkiiunit| 32 byte waveform memory oscillator |

## Build

Clone repository:

```bash
git clone --recursive https://github.com/h1romas4/nts1mkii-rust-wave
```

Install `gcc-arm-none-eabi-10.3` toolchaine:

This project uses the gcc-ld and thumbv7em sysroot provided by gcc-arm-none-eabi. [Install](https://github.com/korginc/logue-sdk/tree/master/tools/gcc) them according to the OS on which the build will be executed.

Linux:

```bash
cd nts1mkii-rust-wave/toolchain
rm -Rf gcc-arm-none-eabi/
wget https://developer.arm.com/-/media/Files/downloads/gnu-rm/10.3-2021.10/gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2
tar xvf gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2
rm gcc-arm-none-eabi-10.3-2021.10-x86_64-linux.tar.bz2
mv gcc-arm-none-eabi-10.3-2021.10 gcc-arm-none-eabi # rename directory for .cargo/config
touch gcc-arm-none-eabi/EMPTY # for git
cd ..
```

macOS:

```bash
cd nts1mkii-rust-wave/toolchain
rm -Rf gcc-arm-none-eabi/
wget https://developer.arm.com/-/media/Files/downloads/gnu-rm/10.3-2021.10/gcc-arm-none-eabi-10.3-2021.10-mac.tar.bz2
tar xvf gcc-arm-none-eabi-10.3-2021.10-mac.tar.bz2
rm gcc-arm-none-eabi-10.3-2021.10-mac.tar.bz2
mv gcc-arm-none-eabi-10.3-2021.10 gcc-arm-none-eabi # rename directory for .cargo/config
touch gcc-arm-none-eabi/EMPTY # for git
cd ..
```

Windows 11 pwsh:

```pwsh
cd nts1mkii-rust-wave/toolchain
curl -L https://developer.arm.com/-/media/Files/downloads/gnu-rm/10.3-2021.10/gcc-arm-none-eabi-10.3-2021.10-win32.zip -o gcc-arm-none-eabi-10.3-2021.10-win32.zip
Expand-Archive -Path ".\gcc-arm-none-eabi-10.3-2021.10-win32.zip"
Copy-Item -Path ".\gcc-arm-none-eabi-10.3-2021.10-win32\gcc-arm-none-eabi-10.3-2021.10\*" -Destination ".\gcc-arm-none-eabi" -Recurse
Remove-Item -Path ".\gcc-arm-none-eabi-10.3-2021.10-win32" -Recurse -Force
Remove-Item .\gcc-arm-none-eabi-10.3-2021.10-win32.zip -Force
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

Transfer `dist/*.nts1mkiiunit` to NTS-1 digital kit mkII.

See [GitHub Actions](https://github.com/h1romas4/nts1mkii-rust-wave/blob/main/.github/workflows/build-release.yml
) for build details.

Enjoy!

## How to add a new sound unit

ex. `osc_hello`

```bash
# add project to workspace
$ cargo new components/osc_hello
# copy template
$ cp -p components/osc_dummy/src/*.rs components/osc_hello/src
# update template
$ rm components/osc_hello/src/main.rs
$ cp -p components/osc_dummy/Cargo.toml components/osc_hello
$ sed -i 's/name = "osc_dummy"/name = "osc_hello"/' components/osc_hello/Cargo.toml
# add to xtask UNIT_NAME xtask/src/build.rs
$ code -g $(grep -Hn UNIT_NAME: xtask/src/build.rs | cut -d':' -f1,2)
const UNIT_NAME: [&str; 4] = [ // increase the number of array elements
    "osc_waves",
    "osc_dummy",
    "osc_table32",
    "osc_hello", // add this line
];
```

Sound unit Settings:

```bash
# edit attribute components/osc_hello/src/header.rs
$ code -g $(grep -Hn dev_id: components/osc_hello/src/header.rs | cut -d':' -f1,2)
    dev_id: sound_unit_dev_id_string!(b"H1RO"),
    // ID for this unit. Scoped within the context of a given dev_id.
    unit_id: 0x050401,
    // Name for this unit, will be displayed on device
    name: sound_unit_string!(b"DMMY", 20),
```

Build:

```bash
$ cargo build --release --target=thumbv7em-none-eabihf
$ cargo xtask dist
$ ls -laF dist/osc_hello.nts1mkiiunit
-rwxrwxr-x 1 hiromasa hiromasa 28208  6月 21 19:18 dist/osc_hello.nts1mkiiunit*
```
## License

BSD-3-Clause License

## Dependencies

Thanks for all the open source.

|Name|Version|License|Note|
|-|-|-|-|
|[logue-sdk](https://github.com/korginc/logue-sdk)|`67ad379`|BSD-3-Clause||

---
## WIP

- [ ] `osc_waves`
    - [x] `K_SHAPE`, `K_SUB_MIX` is not working properly. The setpoints are not being evaluated correctly.
    - [ ] Does not implement `dither` and `bit_res_recip`.

## Build (with libnts1mkii.a and bindgen)

This repository already contains a pre-build of the louge-sdk library and bindings, so there is essentially no need to do this build.

- `components/logue_bind/dist/libnts1mkii.a`
- `components/logue_bind/src/bindings_libnts1mkii.rs`

If you build, please do so under Linux, as cmake is tuned for Linux.

Build logue-sdk: `components/logue_bind/dist/libnts1mkii.a`

```bash
# remove inline attribute for libnts1mkii.a
pushd components/logue-sdk
git apply ../logue_bind/script/louge-sdk-remove-inline.patch
popd
# build
pushd components/logue_bind
mkdir build && cd build
cmake ..
make
cd ..
nm dist/libnts1mkii.a | tee dist/libnts1mkii.obj.txt
popd
```

Build Rust with bindgen: `components/logue_bind/src/bindings_libnts1mkii.rs`

```bash
WITH_LOGUE_SDK_BINDGEN=true cargo build --release --target=thumbv7em-none-eabihf
```

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
 5c0:   0000            movs    r0, r0          ; ?
 5c2:   0000            movs    r0, r0          ; ?
 5c4:   f240 2c1c       movw    ip, #540        ; 0x21c
 5c8:   f2c0 0c00       movt    ip, #0
 5cc:   44fc            add     ip, pc
 5ce:   Address 0x00000000000005ce is out of bounds.
```

Passing the `--long-plt` option to linker fixes it, but not sure if it is correct.

```asm
476c <osc_white@plt>:
476c:   0000        movs    r0, r0              ; ?
476e:   0000        movs    r0, r0              ; ?
4770:   f240 2c40   movw    ip, #576            ; 0x240
4774:   f2c0 0c00   movt    ip, #0
4778:   44fc        add	    ip, pc
477a:   f8dc f000   ldr.w	pc, [ip]
477e:   e7fc        b.n	477a <osc_white@plt+0xe>
```

This repository uses this approach.

### Original louge-sdk gcc & arm-none-eabi-ld ver

Good work.

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
