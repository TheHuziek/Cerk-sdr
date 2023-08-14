# cerk-sdr open-source firmware for malahit-sdr dsp1

this is a project to make an open firmware for the malahit-sdr dsp1 device written in rust

## TODO

* [ ] graphic library for ili9488
* [ ] library for the radio tuner msi001
* [ ] library for nau8822 audio chip

## how to build and load on your malahit 

you need the cargo-binutils

```
$ rustup component add llvm-tools-preview
```

compile the firmware with cargo

```
cargo build 
cargo objcopy --bin cerk-sdr --release -- -O binary cerk-sdr.bin

```

load on your malahit

```
sudo dfu-util -R -a 0 -s 0x8000000 -D cerk-sdr.bin
```
