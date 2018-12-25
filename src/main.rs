/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/
extern crate chrono;
use chrono::{Local, Timelike};
use std::process::Command;


extern crate awaken_pi;

/**
This capture method uses the canned `raspistill` command to
capture still images using the best available settings.
This works slightly better for the Pi camera than using the
rscam abstraction layers.

*/
fn capture_raspistill(filename: &str) {

  //raspistill -v -n -rot 180 -o
  let status = Command::new("raspistill")
	.arg("-n")
  .arg("-rot").arg("180")
	.arg("-t").arg("250")
	.arg("-o").arg(filename)
	.status().expect("cmd failed!");

  if !status.success()  {
    println!("cmd failed {}", status);
  }
  else {
    println!("wrote {}", filename);
  }

}


fn main() {
  use std::path::Path;

  let now = Local::now();
  if now.hour() > 4 && now.hour() < 19 {
    let time_str = now.format("%Y%m%d_%H%M%SZ-cap.jpg").to_string();
    let fname = time_str.clone();
    capture_raspistill(&fname);
  }

  // This app is run as a service at reboot:
  // as a means to stop it running and shutting down forever,
  // check whether an SD card is inserted (via USB OTG)
  // and only set a reboot time if there is no external SD card
  if !Path::new("/dev/sda").exists() {
    awaken_pi::reawaken_in_minutes(5); 
  }
  else { 
    println!("stop running: sd card inserted");
    println!("------- done ----");
  }

  
}
