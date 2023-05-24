#!/bin/sh

apt-get update && apt-get install -y --no-install-recommends apt-utils
dpkg --add-architecture armhf
apt-get install -y pkg-config
apt-get install -y libfreetype6-dev:armhf
apt-get install -y libfontconfig1-dev
