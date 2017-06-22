# Setting up the Raspberry Pi

## Hardware selection

We used a Raspberry Pi 3 Model B, but most of this guide should work with other models, too. Here is everything you will need:
* Raspberry Pi
* A computer running some version of GNU/Linux, and sudo rights on that machine (BSD-likes including OSX should work, too, but some commands may be slightly different)
* A MicroSD card with at least 4GB of storage
* A MicroSD card reader of some kind
* A wifi network with internet access

## Formatting the MicroSD card

The SD card must be formatted FAT32. Most cards will already be formatted this way when you buy them, but here's a short guide, just in case.
Start by opening a terminal, then do the following:

1. `sudo fdisk -l` to list devices. Identify the device name for your SD card (mine was mmcblk0).
2. `sudo fdisk /dev/mmcblk0` (Replace `mmcblk0` with your device name). This will launch fdisk, a disk partition tool.
3. Type `d` to delete a partition. Keep deleting partitions until none remain.
4. Type `n` to make a new partition. Use all the default options so the partition is sized to fill the available space. Make sure to finish with the new partition prompt before continuing.
5. Type `t` to change the partition type, then choose `b` for FAT32.
6. Type `a` to make the card bootable.
7. Type `p` to list the partitions. Make sure everything is correct. There should be one FAT32 partition, sized to the size of the card, and it should be bootable.
8. Type `w` to write and exit back to the terminal.
9. `sudo fdisk -l` again to list devices. You should now see a partition name under the device name for your sd card (mine was mmcblk0p1).
10. `sudo mkfs.vfat /dev/mmcblk0p1` (Replace `mmcblk0p1` with your partition name). This will format your partition as FAT32.

## Flashing the OS image to the MicroSD card

For now, rcgc will be using the Raspbian Lite image, available at https://www.raspberrypi.org/downloads/raspbian/ . Eventually we will host our own preconfigured image.

1. Make sure your MicroSD card is formatted correctly and plugged into your computer.
2. Open a terminal, then do `wget -O raspbian_lite.zip https://downloads.raspberrypi.org/raspbian_lite_latest`
3. `sha1sum raspbian_lite.zip` This will output a string of letters and numbers. Make sure it matches the string after "SHA-1" on the [raspbian download page](https://www.raspberrypi.org/downloads/raspbian/).
4. `unzip raspbian_lite.zip` and note the name of the file that gets inflated (mine was 2017-04-10-raspbian-jessie-lite.img)
5. `lsblk` will list storage devices on your machine. Identify the MicroSD card device name (mine was mmcblk0)
6. `sudo dd bs=4M if=2017-04-10-raspbian-jessie-lite.img of=/dev/mmcblk0` (replace the file name after `if=` with your img file name and the device name after `of=/dev/` with your MicroSD card device name). This command will write the contents of the image to the MicroSD card. `bs=4M` sets the block size of the write operation to 4 MB. This should work for most users, but if it doesn't try lowering the block size.

   Note that this command may take a while to run, and there is no progress indicator. Don't panic.
7. `sync` to flush the write cache before removing the MicroSD card.
8. At this point, the MicroSD card is ready to be plugged into the Raspberry Pi! The next few steps just verify that the image copied successfully. If you like living on the edge, you can skip to step 9.
   1. `sudo dd bs=4M if=/dev/mmcblk0 of=from-sd-card.img` (replace the device name after `if=/dev/` with your MicroSD card device name).
   2. `sudo truncate --reference 2017-04-10-raspbian-jessie-lite.img from-sd-card.img` (replace the file name after `--reference` with your (downloaded & unzipped) img file name)
   3. `diff -s from-sd-card.img 2017-04-10-raspbian-jessie-lite.img` (again, replace the second file name with your (downloaded & unzipped) img file name). This should output `files from-sd-card.img and 2017-04-10-raspbian-jessie-lite.img are identical`.
   4. `sync` again.
9. `umount /dev/mmcblk0` (replace with your MicroSD card device name).
10. Unplug the MicroSD card.

## Configuring the Raspberry Pi

1. Plug the MicroSD card into your Raspberry Pi.
2. Connect (at a minimum) a keyboard and monitor to your Raspberry Pi.
3. Plug in your power supply. The Pi should boot up automatically and eventually show you a login prompt.
4. Log in with the default credentials (login: pi pw: raspberry)
5. The default keyboard layout is "gb" for Great Britain, a country that isn't even called that anymore. Change it by doing `sudo dpkg-reconfigure keyboard-configuration` and following the prompts.
6. `sudo raspi-config` will show a prompt with some other useful options. Configure your internationalization options and your network hostname, then `Expand Filesystem` and exit.
7. Connect to your wifi network:
   1. `sudo nano /etc/wpa_supplicant/wpa_supplicant.conf`
      1. Change country to your country code
      2. Add the following to the end of the file, replacing "your-ssid" and "your-password" with your network name and password (but keep the double quotes!):

          ```
network={
    ssid="your-ssid"
    psk="your-password"
}
          ```

      3. ctrl+O to write your changes, then ctrl+X to exit to the terminal.
   2. `sudo wpa_cli reconfigure`
   3. You should now be connected to your wifi network. `sudo ifconfig wlan0` will show information about your wireless card. If there is an ip address after `inet`, you are connected.
   4. If your wifi network is working right, you should also be connected to the internet. `ping 8.8.8.8` will get some responses if this is the case. (ctrl-c to stop pinging).
8. `sudo apt-get update`
9. `sudo apt-get upgrade`

TODO: create a new user with sudo privileges and delete the default user.

Boot quietly:
`sudo cp /etc/systemd/system/getty.target.wants/getty@tty{1,3}.service`
`sudo nano /etc/systemd/system/getty.target.wants/getty@tty3.service` and change `DefaultInstance=tty1` to `DefaultInstance=tty3`
`sudo nano /boot/cmdline.txt` and make it look like this:
```
dwc_otg.lpm_enable=0 console=serial0,115200 console=tty3 loglevel=0 root=/dev/mmcblk0p2 rootfstype=ext4 elevator=deadline fsck.repair=yes rootwait logo.nologo quiet splash
```

Run the launcher on the chooser at boot:
Add the following to `/etc/rc.local`, above `exit 0` and below everything else:
```
/home/pi/rcgc/rcgc_launcher /home/pi/rcgc/rcgc_chooser/
```
(change these to the correct path to the `rcgc_launcher` binary and the `rcgc_chooser` directory)
