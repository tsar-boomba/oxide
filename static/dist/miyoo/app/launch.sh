#!/bin/sh
export WLD=/mnt/SDCARD/miyoo/app/lib/wayland
export LD_LIBRARY_PATH=/mnt/SDCARD/miyoo/app/lib:$WLD/lib/arm-linux-gnueabihf:$WLD/lib/arm-linux-gnueabihf/weston:/mnt/SDCARD/miyoo/app/lib:$LD_LIBRARY_PATH
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

ln -s $WLD /tmp/wayland

mkdir -p $XDG_RUNTIME_DIR
chmod 0700 $XDG_RUNTIME_DIR

# perform model-specific logic
if [ -f /customer/app/axp_test ]; then
	# Mini Plus
	export MODEL=354

	# kill default telnetd. Allium will launch its own if needed
	killall telnetd
else
	# OG Mini
	export MODEL=283

	# init charger detection
	if [ ! -f /sys/devices/gpiochip0/gpio/gpio59/direction ]; then
		echo 59 > /sys/class/gpio/export
		echo in > /sys/devices/gpiochip0/gpio/gpio59/direction
	fi
fi

# use audioserver to prevent pop-noise
if [ -f /customer/lib/libpadsp.so ]; then
	LD_PRELOAD=as_preload.so audioserver_"$MODEL" &
	export LD_PRELOAD=/customer/lib/libpadsp.so:$LD_PRELOAD
fi

#sleep 30 && sync && poweroff &

# TODO: remove
sleep 3

ls -al /proc > proc.txt
# debug stuff
echo $PATH > path.txt
ls /dev > dev.txt
uname -a > info.txt
ls -a /customer/app > customer_dir.txt
ls -a /sys > sys.txt
ls -a /sys/power > sys_pwr.txt
ls -a /tmp > tmp.txt

# Start weston which will launch the actual OS when it is ready
weston --debug --tty=$WESTON_TTY --config=/mnt/SDCARD/miyoo/app/weston.ini 1> weston.log 2>&1

#sleep 1
#sleep 10 && sync && poweroff &
#emulator /mnt/SDCARD/Cores/vba_next_libretro.so "/mnt/SDCARD/Games/GBA/metroid.gba" 1> launchlog.log 2>&1

# never let the built-in firmware start (it is cringe 💀)
sync
poweroff
sleep 10
