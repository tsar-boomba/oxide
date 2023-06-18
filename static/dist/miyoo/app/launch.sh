#!/bin/sh
export WLD=/mnt/SDCARD/miyoo/app/lib/wayland
export LD_LIBRARY_PATH="/mnt/SDCARD/miyoo/app/lib:$WLD/lib/arm-linux-gnueabihf:$WLD/lib/arm-linux-gnueabihf/weston:$LD_LIBRARY_PATH"
export PATH=$WLD/bin:$PATH
export FONTCONFIG_PATH=/mnt/SDCARD/miyoo/app
export FONTCONFIG_FILE=/mnt/SDCARD/miyoo/app/fonts.conf

# gfx stuff
export ICED_BACKEND=tiny-skia
export WESTON_TTY=1
export WAYLAND_DISPLAY=wayland-1
export XDG_RUNTIME_DIR="/tmp/user/root"
export XKB_CONFIG_ROOT=/mnt/SDCARD/miyoo/app/lib/xkb

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

ln -s $WLD /tmp/wayland

# Start weston which will launch the actual OS when it is ready
chmod u+s $(which weston)
weston --debug --tty=$WESTON_TTY --logger-scopes=log,proto --config=/mnt/SDCARD/miyoo/app/weston.ini 1> weston.log 2>&1

# never let the built-in firmware start (it is cringe 💀)
sync
poweroff
sleep 10
