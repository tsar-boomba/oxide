#!/bin/sh
docker build -t oxide-deps -f ./tools/deps.dockerfile .
# if it fails, it already exists probably
docker create --name oxide-deps oxide-deps || true

mkdir -p ./build/lib
mkdir -p ./build/bin

# copy out libs we need to add to the sd card
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libfontconfig.so.1 ./build/lib/
docker cp -L oxide-deps:/lib/arm-linux-gnueabihf/libexpat.so.1 ./build/lib/
docker cp -L oxide-deps:/lib/arm-linux-gnueabihf/libuuid.so.1 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libX11.so.6 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libXcursor.so.1 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libXrandr.so.2 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libXi.so.6 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libXau.so.6 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libxcb.so.1 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libXdmcp.so.6 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libbsd.so.0 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libXrender.so.1 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libXfixes.so.3 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libXext.so.6 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libX11-xcb.so.1 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libgbm.so.1 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libxkbcommon.so.0 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libwayland-egl.so.1 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libwayland-client.so.0 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libdrm.so.2 ./build/lib/
docker cp -L oxide-deps:/lib/arm-linux-gnueabihf/libudev.so.1 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libinput.so.10 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libwayland-server.so.0 ./build/lib/
docker cp -L oxide-deps:/usr/lib/arm-linux-gnueabihf/libffi.so.6 ./build/lib/

docker container rm oxide-deps

echo "Got libs:"
ls ./build/lib
