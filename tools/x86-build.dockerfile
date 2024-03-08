FROM debian:buster-slim

RUN dpkg --add-architecture armhf
RUN apt-get update
RUN apt-get install -y libfontconfig1-dev:armhf
RUN apt-get install -y libfreetype6-dev:armhf
RUN apt-get install -y libxcb1-dev:armhf
RUN apt-get install -y libxcb-render0-dev:armhf
RUN apt-get install -y libxcb-shape0-dev:armhf
RUN apt-get install -y libxcb-xfixes0-dev:armhf
RUN apt-get install -y libxkbcommon-dev:armhf
RUN apt-get install -y libwayland-dev:armhf
RUN apt-get install -y libudev-dev:armhf
RUN apt-get install -y libgbm-dev:armhf
RUN apt-get install -y libinput-dev:armhf
RUN apt-get install -y libasound2-dev:armhf
RUN apt-get install -y clang
RUN apt-get install -y g++-arm-linux-gnueabihf
RUN apt-get install -y libc6-dev-armhf-cross
RUN apt-get install -y cmake curl git build-essential

ENV CXXFLAGS="-fPIC"
ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS="-L /usr/lib/arm-linux-gnueabihf -C relocation-model=pic -C link-args=-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf $CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS"
ENV CROSS_TOOLCHAIN_PREFIX=arm-linux-gnueabihf-
ENV CROSS_SYSROOT=/usr/arm-linux-gnueabihf
ENV CROSS_TARGET_RUNNER="/linux-runner armv7hf"
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER="$CROSS_TOOLCHAIN_PREFIX"gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUNNER="$CROSS_TARGET_RUNNER" \
    AR_armv7_unknown_linux_gnueabihf="$CROSS_TOOLCHAIN_PREFIX"ar \
    CC_armv7_unknown_linux_gnueabihf="$CROSS_TOOLCHAIN_PREFIX"gcc \
    CXX_armv7_unknown_linux_gnueabihf="$CROSS_TOOLCHAIN_PREFIX"g++ \
    CMAKE_TOOLCHAIN_FILE_armv7_unknown_linux_gnueabihf=/opt/toolchain.cmake \
    BINDGEN_EXTRA_CLANG_ARGS_armv7_unknown_linux_gnueabihf="--sysroot=$CROSS_SYSROOT" \
    QEMU_LD_PREFIX="$CROSS_SYSROOT" \
    RUST_TEST_THREADS=1 \
    CROSS_CMAKE_SYSTEM_NAME=Linux \
    CROSS_CMAKE_SYSTEM_PROCESSOR=arm \
    CROSS_CMAKE_CRT=gnu \
    CROSS_CMAKE_OBJECT_FLAGS="-ffunction-sections -fdata-sections -fPIC -march=armv7-a -mfpu=vfpv3-d16"