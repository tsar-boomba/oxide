#!/bin/sh

# Copy libraries and such to the build directory for zipping
cp -r ./static/dist/.tmp_update ./build/PAYLOAD
cp -r ./static/dist/miyoo ./build/PAYLOAD
cp ./target/armv7-unknown-linux-gnueabihf/release/os ./build/PAYLOAD/miyoo/app/MainUI
cp ./target/armv7-unknown-linux-gnueabihf/release/compositor ./build/PAYLOAD/miyoo/app/bin/compositor

# copy libs from the deps docker container
cp -r -L ./build/lib ./build/PAYLOAD/miyoo/app/

# copy bins
cp -r -L ./build/bin ./build/PAYLOAD/miyoo/app/

# Copy over libraries from the toolchain
cp -L ./miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/lib/libfreetype.so.6 ./build/PAYLOAD/miyoo/app/lib/
cp -L ./miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/lib/libpng16.so.16 ./build/PAYLOAD/miyoo/app/lib/
cp -L ./miyoomini-toolchain/arm-linux-gnueabihf/libc/usr/lib/libz.so.1 ./build/PAYLOAD/miyoo/app/lib/
cp -L ./miyoomini-toolchain/arm-linux-gnueabihf/libc/lib/libc.so.6 ./build/PAYLOAD/miyoo/app/lib/
