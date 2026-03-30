# xv0
*An operating system, not the best, but its an operating system, kinda*

**This OS is a hobby project, its very minimal and incomplete! Made for the "boot" hackclub YSWS**

## Running it
**NOTE:** I have yet to try it on real hardware, and right now its only recommended to run through qemu, as shown below.

### Requirements
- Linux (Only tested on linux, if on windows, try it on wsl, if it doesnt work on wsl, then... too bad? idk)
- Curl and Git installed
- Rustup
- Qemu (full version, aka with gtk gui)

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

### Running the program
`bootimage` plugs directly into the default run command so we dont have to do anything with `make` (`gmake`) and we can compile the OS and automatically launch qemu with a single command:
```sh
cargo run
```

## Support
If you encounter any issues, please make an github issue! Any bug reports or similar help me improve and polish the OS as i keep developing it.

## License
This project is dual-licensed under the Apache 2.0 and MIT license. 
