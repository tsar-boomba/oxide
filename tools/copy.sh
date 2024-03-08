#!/bin/sh

mkdir -p ./build/PAYLOAD/Saves

# Copy libraries and such to the build directory for zipping
cp -r ./static/dist/.tmp_update ./build/PAYLOAD
cp -r ./static/dist/miyoo ./build/PAYLOAD
cp ./target/armv7-unknown-linux-gnueabihf/$1/os ./build/PAYLOAD/miyoo/app/MainUI

# copy libs from the deps docker container
cp -r -L ./build/lib ./build/PAYLOAD/miyoo/app/
cp -r -L ./build/cores ./build/PAYLOAD/Cores/

# copy bins
cp ./target/armv7-unknown-linux-gnueabihf/$1/emulator ./build/bin/emulator
cp -r -L ./build/bin ./build/PAYLOAD/miyoo/app/

# Copy over libraries from the toolchain
cp -L ./miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/lib/libfreetype.so.6 ./build/PAYLOAD/miyoo/app/lib/
cp -L ./miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/lib/libpng16.so.16 ./build/PAYLOAD/miyoo/app/lib/
cp -L ./miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/lib/libz.so.1 ./build/PAYLOAD/miyoo/app/lib/
cp -L ./miyoomini-toolchain/arm-linux-gnueabihf/libc/lib/libc.so.6 ./build/PAYLOAD/miyoo/app/lib/
