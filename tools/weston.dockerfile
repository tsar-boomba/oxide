FROM arm32v7/debian:buster-slim

# Build weston for armv7 following https://wayland.freedesktop.org/building.html

RUN echo "deb http://deb.debian.org/debian buster-backports main" >> /etc/apt/sources.list.d/backports.list
RUN apt-get update

RUN apt-get install -y python3
RUN apt-get install -y python3-pip
RUN pip3 install meson

RUN apt-get install -y ninja-build
RUN apt-get install -y build-essential
RUN apt-get install -y git
RUN apt-get install -y cmake/buster-backports
RUN apt-get install -y pkg-config


ENV WLD=/tmp/wayland
# Add the wayland libs we just build to pkg-config
ENV PKG_CONFIG_PATH=$WLD/lib/arm-linux-gnueabihf/pkgconfig/:$WLD/share/pkgconfig/:$PKG_CONFIG_PATH

# Build wayland
RUN git clone --branch 1.22.0 https://gitlab.freedesktop.org/wayland/wayland.git

RUN apt-get install -y libffi-dev
RUN apt-get install -y libxml2-dev

RUN cd wayland && meson build/ --prefix=$WLD -Ddocumentation=false
RUN cd wayland && ninja -C build/ install

# Build wayland protocols
RUN git clone --branch 1.31 https://gitlab.freedesktop.org/wayland/wayland-protocols.git
RUN cd wayland-protocols && meson build/ --prefix=$WLD
RUN cd wayland-protocols && ninja -C build/ install

RUN git clone --depth 1 --branch 10.0.4 https://gitlab.freedesktop.org/wayland/weston.git

RUN apt-get install -y libxkbcommon-dev
RUN apt-get install -y libpixman-1-dev
RUN apt-get install -y libinput-dev
RUN apt-get install -y libdrm-dev
RUN apt-get install -y libcairo2-dev
RUN apt-get install -y liblcms2-dev
RUN apt-get install -y libxcb-composite0-dev
RUN apt-get install -y libxcursor-dev
RUN apt-get install -y libpam0g-dev

RUN cd weston && meson build/ --prefix=$WLD -Ddeprecated-backend-fbdev=true \
	-Dbackend-drm=false -Dbackend-drm-screencast-vaapi=false \
	-Dbackend-rdp=false -Dscreenshare=false -Dbackend-wayland=false -Dbackend-x11=false \
	-Dbackend-default=headless -Drenderer-gl=false \
	-Dsystemd=false -Dremoting=false -Dpipewire=false -Dshell-ivi=false \
	-Ddeprecated-wl-shell=true -Dcolor-management-colord=false -Dlauncher-logind=false \
	-Ddemo-clients=false -Dsimple-clients=[] -Dresize-pool=false -Dwcap-decode=false \
	-Dtest-junit-xml=false -Dtest-gl-renderer=false -Dimage-jpeg=false -Dimage-webp=false \
	-Ddeprecated-weston-launch=true
RUN cd weston && ninja -C build/ install
