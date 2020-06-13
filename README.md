

## What

Enables using a DS3231 standalone real time clock to repeatedly halt and reawaken a Raspberry Pi. This allows you to maximize your battery life while doing things like time lapse photography. 

Tested with rpi3 and rpi0w.

Wiring instructions coming soon.  Briefly: Put a 4.7nF capacitor between INT pin of the DS3231 module and the SCLK ("C") pin connected to the rpi. This acts as an RC differentiator, sending a single short pulse to the SCLK pin on the rpi when the INT pin latches low. If you simply connect INT directly to SCLK, this would interfere with the i2c bus and prevent communications with the DS3231. 

### Enabling i2c

- Use `sudo raspi-config` to enable i2c (raspberry pi only)
- Check `ls /dev/*i2c*` and verify `/dev/i2c-1` is visible
- Install i2c tools:  `sudo apt-get install -y i2c-tools`
- Attach a [suitable DS3231 RTC module](http://a.co/d/0KolyPX) and check that `sudo i2cdetect -y 1` shows a device at address `68`

#### Note on rock64
- Enabling gpio and i2c on rock64 is slightly different. By default Armbian Buster enables i2c1 and i2c4.  
- You can connect the RTC to i2c1 on pins 27 (SDA) and 28 (SCL) and then `i2cdetect` should find the device as above.


### First time run

- Try `cargo run` and verify that the binary takes a picture with the rpi camera, then shuts down. A few minutes time later, the rpi should reboot.  If you setup the systemd service as detailed below, the app will automatically restart at boot, giving you a timelapse photography service. 

### Setting up service to start at reboot
- Build `awaken_pi` with `cargo build`
- Edit and copy `/extras/timecam.service` to `/lib/systemd/system/timecam.service`: Note that the built location of your `awaken_pi` binary will likely differ from the absolute path given in that file.  Adjust `ExecStart` and  `WorkingDirectory` as desired.
- Call `sudo systemctl enable timecam.service`


### License
See LICENSE file
