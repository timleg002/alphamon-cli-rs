_list:
    just --list

build-armv7:
    cross build --target armv7-unknown-linux-gnueabihf

build-armv8:
    cross build --target aarch64-unknown-linux-gnueabihf