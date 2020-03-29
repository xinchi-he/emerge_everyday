# Emerge EveryDay

This is a simple program to parse the emerge log for the Gentoo Linux.

## Sample Output
```sh
[~/emerge_everyday]$ go run ./eed.go
********* Monthly Emerges **********
03/2019,  emerges: 246
04/2019,  emerges: 154
05/2019,  emerges: 130
03/2020,  emerges: 103
01/2020,  emerges: 99
02/2020,  emerges: 90
11/2019,  emerges: 76
06/2019,  emerges: 73
12/2019,  emerges: 73
10/2019,  emerges: 60
07/2019,  emerges: 53
09/2019,  emerges: 48
08/2019,  emerges: 23

********** Top Emerged Packages (more than 10 times) **********
www-client/google-chrome 27
net-im/telegram-desktop-bin 24
www-client/firefox-bin 23
media-libs/mesa 19
sys-kernel/linux-firmware 18
media-gfx/imagemagick 18
dev-lang/python 17
app-emulation/virtualbox-modules 16
dev-lang/ruby 14
media-libs/alsa-lib 14
sys-apps/portage 13
sys-apps/flatpak 12
media-plugins/alsa-plugins 12
app-emulation/docker 12
net-wireless/blueman 12
sys-kernel/gentoo-sources 12
dev-python/cryptography 12
app-text/docbook-xml-dtd 11
media-sound/alsa-utils 11
dev-lang/go 11
dev-libs/glib 11
sys-devel/llvm 11
net-wireless/bluez 11
dev-python/cffi 10
net-misc/dropbox 10
dev-python/pygobject 10
sys-libs/glibc 10
dev-util/meson 10
dev-python/pygments 10
x11-libs/gtk+ 10
dev-vcs/git 10
```