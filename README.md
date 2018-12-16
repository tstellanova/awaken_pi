

## What

Enables using a DS3231 standalone real time clock to repeatedly halt and reawaken a Raspberry Pi. This allows you to maximize your battery life while doing things like time lapse photography. 

Tested with rpi3 and rpi0w.

Wiring instructions coming soon.  Briefly: Put a 4.7nF capacitor between INT pin of the DS3231 and the SCLK ("C") pin connected to the rpi. This acts as an RC differentiator, sending a single short pulse to the SCLK pin on the rpi when the INT pin latches low. If you simply connect INT directly to SCLK, this would interfere with the i2c bus and prevent communications with the DS3231. 

### Enabling i2c

- Use `sudo raspi-config` to enable i2c
- Check `ls /dev/*i2c*` and verify `/dev/i2c-1` is visible
- Attach a suitable DS3231 RTC module and check that `sudo i2cdetect -y 1` shows a device at address `68`

### First time run

- Try `cargo run` and verify that the binary takes a picture with the rpi camera, then shuts down. A few minutes time later, the rpi should reboot.  If you setup the systemd service as detailed below, the app will automatically restart at boot, giving you a timelapse photography service. 

### Setting up service to start at reboot
- Build `awaken_pi` with `cargo build`
- Edit and copy `/extras/timecam.service` to `/lib/systemd/system/timecam.service`: Note that the built location of your `awaken_pi` binary will likely differ from the absolute path given in that file.  Adjust `ExecStart` and  `WorkingDirectory` as desired.
- Call `sudo systemctl enable timecam.service`


### License
See LICENSE file
