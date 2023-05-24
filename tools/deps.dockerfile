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
RUN apt-get install -y libwayland-egl1:armhf
RUN apt-get install -y libwayland-client0:armhf
RUN apt-get install -y libdrm2:armhf
RUN apt-get install -y libudev1:armhf
RUN apt-get install -y libinput10:armhf
RUN apt-get install -y libwayland-server0:armhf
RUN apt-get install -y libffi6:armhf
