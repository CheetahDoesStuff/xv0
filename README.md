# xv0
*An operating system, not the best, but its an operating system, kinda*

**This OS is a hobby project, its very minimal and incomplete! Made for the "boot" hackclub YSWS**

## Running it - From release
**NOTE:** I have yet to try it on real hardware, and right now its only recommended to run through qemu, as shown below.

### Getting the image
Just go to the [latest release](https://github.com/CheetahDoesStuff/xv0/releases/latest) and download the .img file!

### Run with qemu
Run the file in qemu (replace the file parameter with your actual filename):
```sh
qemu-system-x86_64 -drive format=raw,file=xv0-release-VERSION.img
```

## Running it - From source
**NOTE:** I have yet to try it on real hardware, and right now its only recommended to run through qemu, as shown below.

### Requirements
- Linux (Only tested on linux, if on windows, try it on wsl, if it doesnt work on wsl, then... too bad? idk)
- Curl and Git installed
- Rustup
- Qemu (full version, aka with gtk gui)
- Doom1 WAD (I recommend freedoom, which is whats used for prebuilt releases)

### Cloning the repo
Just clone the repo:
```sh
git clone https://github.com/CheetahDoesStuff/xv0
cd xv0
```

### Setting up the enviorment
We have to switch the rust version to nightly to compile the program, as well as install the required `bootimage` crate that xv0 uses to build its bootable image:
```sh
rustup default nightly
cargo install bootimage
```

To be able to build the program you also need to provide some sort of DOOM wad file, this can be any DOOM wad file, but its recommended to use one of the freedoom wad files. Simply put your WAD file in the root directory and rename it to `doom1.wad`.

### Running the program
`bootimage` plugs directly into the default run command so we dont have to do anything with `make` (`gmake`) and we can compile the OS and automatically launch qemu with a single command:
```sh
cargo run --release
```

## Support
If you encounter any issues, please make an github issue! Any bug reports or similar help me improve and polish the OS as i keep developing it.

## License
This project is dual-licensed under the Apache 2.0 and MIT license. 
