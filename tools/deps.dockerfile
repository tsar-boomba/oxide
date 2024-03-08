FROM debian:buster-slim

RUN dpkg --add-architecture armhf
RUN apt-get update
RUN apt-get install -y libfontconfig1:armhf
RUN apt-get install -y libexpat1:armhf
RUN apt-get install -y libuuid1:armhf
RUN apt-get install -y libx11-6:armhf
RUN apt-get install -y libxcursor1:armhf
RUN apt-get install -y libxrandr2:armhf
RUN apt-get install -y libxi6:armhf
RUN apt-get install -y libxau6:armhf
RUN apt-get install -y libxcb1:armhf
RUN apt-get install -y libxdmcp6:armhf
RUN apt-get install -y libbsd0:armhf
RUN apt-get install -y libxrender1:armhf
RUN apt-get install -y libxfixes3:armhf
RUN apt-get install -y libxext6:armhf
RUN apt-get install -y libx11-xcb1:armhf
RUN apt-get install -y libgbm1:armhf
RUN apt-get install -y libxkbcommon0:armhf
RUN apt-get install -y libdrm2:armhf
RUN apt-get install -y libudev1:armhf
RUN apt-get install -y libinput10:armhf
RUN apt-get install -y libffi6:armhf
RUN apt-get install -y libcairo2:armhf
RUN apt-get install -y liblcms2-2:armhf
RUN apt-get install -y libxcb-composite0:armhf
RUN apt-get install -y libevdev2:armhf
RUN apt-get install -y libpixman-1-0:armhf
RUN apt-get install -y libmtdev1:armhf
RUN apt-get install -y libwacom2:armhf
RUN apt-get install -y libgudev-1.0-0:armhf
RUN apt-get install -y libglib2.0-0:armhf
RUN apt-get install -y libpcre3:armhf
RUN apt-get install -y libmount1:armhf
RUN apt-get install -y libblkid1:armhf
RUN apt-get install -y libselinux1:armhf
RUN apt-get install -y libpam0g:armhf
RUN apt-get install -y libaudit1:armhf
RUN apt-get install -y libcap-ng0:armhf
RUN apt-get install -y libxcb-shm0:armhf
RUN apt-get install -y libxcb-render0:armhf
RUN apt-get install -y libasound2:armhf
