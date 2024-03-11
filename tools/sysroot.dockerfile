FROM arm32v7/debian:buster-slim

RUN apt-get update
RUN apt-get install -y libfontconfig1
RUN apt-get install -y libexpat1
RUN apt-get install -y libuuid1
RUN apt-get install -y libx11-6
RUN apt-get install -y libxcursor1
RUN apt-get install -y libxrandr2
RUN apt-get install -y libxi6
RUN apt-get install -y libxau6
RUN apt-get install -y libxcb1
RUN apt-get install -y libxdmcp6
RUN apt-get install -y libbsd0
RUN apt-get install -y libxrender1
RUN apt-get install -y libxfixes3
RUN apt-get install -y libxext6
RUN apt-get install -y libx11-xcb1
RUN apt-get install -y libgbm1
RUN apt-get install -y libxkbcommon0
RUN apt-get install -y libdrm2
RUN apt-get install -y libudev1
RUN apt-get install -y libinput10
RUN apt-get install -y libffi6
RUN apt-get install -y libcairo2
RUN apt-get install -y liblcms2-2
RUN apt-get install -y libxcb-composite0
RUN apt-get install -y libevdev2
RUN apt-get install -y libpixman-1-0
RUN apt-get install -y libmtdev1
RUN apt-get install -y libwacom2
RUN apt-get install -y libgudev-1.0-0
RUN apt-get install -y libglib2.0-0
RUN apt-get install -y libpcre3
RUN apt-get install -y libmount1
RUN apt-get install -y libblkid1
RUN apt-get install -y libselinux1
RUN apt-get install -y libpam0g
RUN apt-get install -y libaudit1
RUN apt-get install -y libcap-ng0
RUN apt-get install -y libxcb-shm0
RUN apt-get install -y libxcb-render0
RUN apt-get install -y libasound2 libasound2-dev
RUN apt-get install -y libc6-dev
RUN apt-get install -y build-essential
