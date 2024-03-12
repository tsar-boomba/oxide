#!/bin/sh
docker build -t oxide-sysroot -f ./tools/sysroot.dockerfile .
# if it fails, it already exists probably
docker create --name oxide-sysroot oxide-sysroot || true

mkdir -p ./build/lib
mkdir -p ./build/bin

# copy out libs we need to add to the sd card
rm -rf ./build/sysroot
docker cp oxide-sysroot:/ ./build/sysroot

./tools/fixSymlinkLibs.ts

docker container rm oxide-sysroot
