#!/bin/sh
export WLD=/mnt/SDCARD/miyoo/app/lib/wayland
export LD_LIBRARY_PATH="/mnt/SDCARD/miyoo/app/lib:$WLD/lib/arm-linux-gnueabihf:$WLD/lib/arm-linux-gnueabihf/weston:$LD_LIBRARY_PATH"
export PATH=$WLD/bin:/mnt/SDCARD/miyoo/app/bin:$PATH
export FONTCONFIG_PATH=/mnt/SDCARD/miyoo/app
export FONTCONFIG_FILE=/mnt/SDCARD/miyoo/app/fonts.conf

# init_lcd
cat /proc/ls
sleep 0.25

# init backlight
echo 0 > /sys/class/pwm/pwmchip0/export
echo 800 > /sys/class/pwm/pwmchip0/pwm0/period
echo 6 > /sys/class/pwm/pwmchip0/pwm0/duty_cycle
echo 1 > /sys/class/pwm/pwmchip0/pwm0/enable
echo 0 > /sys/class/pwm/pwmchip0/unexport

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
ls /sys > sys.txt
ls /sys/power > sys_pwr.txt

ln -s $WLD /tmp/wayland

#sleep 30 && sync && poweroff &

# Start weston which will launch the actual OS when it is ready
weston --debug --tty=$WESTON_TTY --config=/mnt/SDCARD/miyoo/app/weston.ini 1> weston.log 2>&1

#sleep 1
#sleep 10 && sync && poweroff &
#emulator /mnt/SDCARD/Cores/vba_next_libretro.so "/mnt/SDCARD/Games/GBA/metroid.gba" 1> launchlog.log 2>&1

# never let the built-in firmware start (it is cringe 💀)
sync
poweroff
sleep 10
