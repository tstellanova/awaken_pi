

## What

Enables using a DS3231 standalone real time clock to repeatedly halt and reawaken a Raspberry Pi. This allows you to maximize your battery life while doing things like time lapse photography. 

Tested with rpi3 and rpi0w.

Wiring instructions coming soon.  Basically put a 4.7nF capacitor between INT pin of the DS3231 and the SCLK ("C") pin connected to the rpi. This acts as an RC differentiator, sending a single short pulse to the SCLK pin on the rpi when the INT pin latches low. If you simply connect INT directly to SCLK, this would interfere with the i2c bus and prevent communications with the DS3231. 


### Setting up service to start at reboot

- Build `awaken_pi` with `cargo build`
- Copy the following file to `/lib/systemd/system/timecam.service`:

```
[Unit]
Description=Timecam Service
After=network.target 

[Service]
Type=simple
User=pi
WorkingDirectory=/home/pi/Documents/proj/awaken_pi
ExecStart=/home/pi/Documents/proj/awaken_pi/target/debug/awaken_pi
Restart=on-abort
StandardOutput=journal
StandardError=inherit
SyslogIdentifier=timecam

[Install]
WantedBy=multi-user.target
```
Note that the built location of your `awaken_pi` binary may differ from the absolute path given above. 

- Call `sudo systemctl enable timecam.service`


### License
See LICENSE file
