HelloPi
=======

This is my hobby project at writing a chat bot for
the [Discord][discord] multi-user chat service, 
written in [Rust][rust] and running on my [Raspberry Pi][raspberrypi].
It is also my first attempt at doing cross compilation,
as I want to write and test the code on my x86 Ubuntu
desktop and laptor but finally deploy it to the ARMv7
based Raspberry Pi. Details on how to set up your Ubuntu
16.04 machine for cross compiling Rust code to the Raspberry
Pi, and how to build the [OpenSSL][openssl] library for 
cross-compilation are included below.

[discord]: https://discordapp.com "Discord"
[rust]: https://www.rust-lang.org "Rust"
[raspberrypi]: https://www.raspberrypi.org "Raspberry Pi"
[openssl]: https://www.openssl.org/ "OpenSSL"

Preparing for cross compilation
===============================

Installing the GCC cross compiler
---------------------------------

We need to install the GCC cross compiler for armv7 so
that the Rust compiler will be able to link our executables,
as well as build any packages that use C bindings, like
the `openssl` crate. I own the Raspberry Pi Model B 2 which
has a 32-bit ARMv7 CPU with a built in floating point unit,
and is running the Raspbian Linux distribution which means
that my target is the `armv7-unknown-linux-gnueabihf` system.
The cross compilation libraries needed for this can be installed
straight from the Ubuntu repos:

```
sudo apt-get install gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf
```

If you have a different Raspberry Pi that doesn't have a built
in FPU, or has a more modern ARM processor then your target triple
and cross compiler may be different.

Installing Rust and the cross compilation target (via rustup)
-------------------------------------------------------------

The easiset way to use multiple versions of [Rust][rust],
or to have multiple compilation targets is to use the
[rustup][rustup] tool. Instructions for installing it can
be found on their website.

Once you have installed `rustup` we need to add the
Rasperry Pi target which is called `armv7-unknown-linux-gnueabihf`.
This can be done by running the following:

```
$ rustup target add armv7-unknown-linux-gnueabihf
```

To verify that it installed correctly you can just run
the command again. You should see something like this:

```
info: component 'rust-std' for target 'armv7-unknown-linux-gnueabihf' is up to date
```

We next need to tell `cargo` where it can find the cross
compiler for ARM that we installed earlier. To do this you
need to add the following lines to your `~/.cargo/config`
file, creating it if it does not already exist:

```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

You should now be able to cross compile Rust code to
target the Raspberry Pi! Remember that you need to pass
the `--target=armv7-unknown-linux-gnueabihf` option to
`cargo` for it to actually cross compile. For example:

```
$ cargo new hello_world --bin
$ cd hello_world
$ cargo build --target=armv7-unknown-linux-gnueabihf
   Compiling hello_world v0.1.0 (file:///home/hello_pi/hello_world)
```

and the resulting executable will be in the
`target/armv7-unknown-linux-gnueabihf/debug` folder.

Compiling OpenSSL for cross compilation
---------------------------------------

OpenSSL generates some header files at compile time that
do not translate across platforms so we need to recompile
OpenSSL if we want to cross compile to the Rasperry Pi.

The first step is to download the OpenSSL source code
and checkout your preferred version

```
$ wget ftp://ftp.openssl.org/source/openssl-1.0.2h.tar.gz
$ tar -xzf openssl-1.0.2h.tar.gz
$ cd openssl-1.0.2h/
```

Next we need to prepare our environment to build targetting
ARM:

```
$ export CROSS=arm-linux-gnueabihf
$ export AR=$CROSS-ar
$ export AS=$CROSS-as
$ export CC=$CROSS-gcc
$ export CXX=$CROSS-g++
$ export LD=$CROSS-ld
$ ./Configure --prefix=/usr/arm-linux-gnueabihf linux-armv4
```

Finally we can build and install the library

```
$ make
$ sudo make install
```

[rustup]: https://www.rustup.rs "Rustup"

