/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/
extern crate chrono;
use chrono::{Timelike};
use std::process::Command;


extern crate awaken_pi;

/**
This capture method uses the canned `raspistill` command to
capture still images using the best available settings.
This works slightly better for the Pi camera than using the
rscam abstraction layers.

*/
fn capture_raspistill(filename: &str) {

  //raspistill -v -n  -o
  let status = Command::new("raspistill")
	.arg("-n")
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
  let now = awaken_pi::get_date_time();    
  let time_str = now.format("%Y%m%d_%H%M%SZ-cap.jpg").to_string();

  let local_hour = now.hour() - 8; //Biased to PST
  if local_hour > 4 && local_hour < 19 {
    let fname = time_str.clone();
    capture_raspistill(&fname);
  }
  else {
      println!("nighttime: skip photo {}",time_str);
  }

  // This app is run as a service at reboot:
  // as a means to stop it running and shutting down forever,
  // check whether an SD card is inserted (via USB OTG)
  // and only set a reboot time if there is no external SD card
    if !Path::new("/dev/sda").exists() {
    awaken_pi::reawaken_in_minutes(5 as u8);
  }
  else {
    println!("stop running: sd card inserted");
    println!("------- done ----");
  }
  
}
