#!/bin/sh
export LD_LIBRARY_PATH="/mnt/SDCARD/miyoo/app/lib:$LD_LIBRARY_PATH"

# gfx stuff
export ICED_BACKEND=tiny-skia
export WAYLAND_DISPLAY=wayland-0
export XDG_RUNTIME_DIR="/tmp/user/$(id -u)"
mkdir -p $XDG_RUNTIME_DIR
chmod 0700 $XDG_RUNTIME_DIR

# debug stuff
echo $PATH > path.txt
ls /dev > dev.txt
uname -a > info.txt
ls /customer/app > customer_dir.txt

# init backlight
echo 0 > /sys/class/pwm/pwmchip0/export
echo 800 > /sys/class/pwm/pwmchip0/pwm0/period
echo 6 > /sys/class/pwm/pwmchip0/pwm0/duty_cycle
echo 1 > /sys/class/pwm/pwmchip0/pwm0/enable

# init lcd
cat /proc/ls
sleep 0.25

./bin/compositor 1> compositor.log 2>&1 &

# log stdout and stderr
RUST_BACKTRACE=full ./MainUI 1> my_wogs.log 2>&1 || true

cp /dev/fb0 myfile.txt

# never let the built-in firmware start (it is cringe 💀)
sync
poweroff
sleep 10
