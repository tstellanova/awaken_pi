/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/
extern crate chrono;
use chrono::{Utc};
use std::process::Command;

use std::thread::sleep;

extern crate awaken_pi;
extern crate route_available;

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
  let now = Utc::now();
  let time_str = now.format("%Y%m%d_%H%M%SZ-cap.jpg").to_string();
  let fname = time_str.clone();
  capture_raspistill(&fname);
  // assuming we are running this application as a service at reboot,
  // we want to give ourselves time to ssh into this machine and stop the
  // service if desired. 
  if route_available::route_available() {
    //wait a little while, give us a chance to ssh in
    sleep(std::time::Duration::from_secs(60));
  }
  
  awaken_pi::reawaken_in_minutes(4);
}
