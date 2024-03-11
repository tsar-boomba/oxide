#!/bin/sh
docker build -t oxide-deps -f ./tools/deps.dockerfile .
# if it fails, it already exists probably
docker create --name oxide-deps oxide-deps || true

mkdir -p ./build/lib
mkdir -p ./build/bin

# copy out libs we need to add to the sd card
docker cp -L oxide-deps:/ ./build/sysroot

SYSROOT=build/sysroot

cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libfontconfig.so.1 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libexpat.so.1 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libuuid.so.1 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libX11.so.6 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libXcursor.so.1 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libXrandr.so.2 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libXi.so.6 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libXau.so.6 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libxcb.so.1 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libXdmcp.so.6 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libbsd.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libXrender.so.1 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libXfixes.so.3 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libXext.so.6 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libX11-xcb.so.1 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libgbm.so.1 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libxkbcommon.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libdrm.so.2 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libudev.so.1 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libinput.so.10 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libffi.so.6 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libcairo.so.2 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/liblcms2.so.2 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libxcb-composite.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libevdev.so.2 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libpixman-1.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libmtdev.so.1 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libwacom.so.2 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libgudev-1.0.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libgobject-2.0.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libglib-2.0.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libgio-2.0.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libgmodule-2.0.so.0 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libpcre.so.3 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libmount.so.1 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libblkid.so.1 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libselinux.so.1 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libpam.so.0 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libpamc.so.0 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libpam_misc.so.0 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libaudit.so.1 ./build/lib
cp -L $SYSROOT/lib/arm-linux-gnueabihf/libcap-ng.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libxcb-shm.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libxcb-render.so.0 ./build/lib
cp -L $SYSROOT/usr/lib/arm-linux-gnueabihf/libasound.so.2 ./build/lib

cp -LR $SYSROOT/usr/share/X11/xkb ./build/lib
cp -LR $SYSROOT/usr/share/pam ./build/lib
cp -LR $SYSROOT/usr/share/pam-configs ./build/lib
cp -LR $SYSROOT/etc/fonts ./build/font-config
cp -LR $SYSROOT/etc/fonts/conf.d/ ./build/font-config/conf.d

docker container rm oxide-deps

echo "Got libs:"
ls ./build/lib
