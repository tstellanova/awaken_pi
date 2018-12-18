/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

#![crate_type = "lib"]





extern crate chrono;
use chrono::{Duration, Datelike, Timelike, Utc};

extern crate runas;
use runas::Command;

extern crate linux_embedded_hal as hal;

extern crate ds323x;
use ds323x::{ Ds323x, Hours, DayAlarm1, Alarm1Matching };



pub fn set_awaken_date_time(date_time: &chrono::DateTime<chrono::Utc>) {
  reawaken_at_time(&date_time);
}


/// Shut down the system, entering HALT mode on raspberry pi
fn safe_shutdown() {
  //shutdown -h now
  let status = Command::new("shutdown")
	.arg("-h").arg("now")
	.status().expect("cmd failed!");

  if !status.success()  {
    println!("shutdown failed {}", status);
  }
  else {
    println!("shutdown");
  }
}


/// Tell the RTC to set an alarm by delay from the current time
fn set_minutes_delay_alarm(minutes_delay: u32) {
  let actual_time = Utc::now();
  let goal = actual_time + Duration::minutes(minutes_delay as i64);
  set_date_time_alarm(&goal);
}

fn set_date_time_alarm(goal: &chrono::DateTime<chrono::Utc>) {
  let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
  let mut rtc = Ds323x::new_ds3231(dev);

  // Verify that the RTC is set to some reasonable time
  let actual_time = Utc::now();
  let cur_date = rtc.get_datetime().expect("Couldn't get the date time");
  if cur_date.year < 2018 {
    println!("bogus cur_date");
    
    let datetime = ds323x::DateTime {
                              year: actual_time.year() as u16,
                              month: actual_time.month() as u8,
                              day: actual_time.day() as u8,
                              weekday: actual_time.weekday() as u8,
                              hour: Hours::H24(actual_time.hour() as u8),
                              minute: actual_time.minute() as u8,
                              second: actual_time.second() as u8,
                   };
                   
    rtc.set_datetime(&datetime).expect("couldn't set_datetime");
  }
    
  // The INT/SQW output will be latched low if the alarm has already fired: clear it
  if rtc.has_alarm1_matched().expect("Couldn't check alarm1") {
    println!("Alarm already fired!");
    rtc.clear_alarm1_matched_flag().expect("couldn't clear alarm1 flag");
  }
  
  // day and hours do not matter, since we're using MinutesAndSecondsMatch below
  let alarm1 = DayAlarm1 {
      day: 1, // unused
      hour: Hours::H24(goal.hour() as u8), 
      minute: goal.minute() as u8,
      second: 1
  };
  
  //  Alarm should fire when minutes and seconds match
  rtc.set_alarm1_day(alarm1, Alarm1Matching::MinutesAndSecondsMatch ).expect("Couldn't set alarm");
  rtc.use_int_sqw_output_as_interrupt().expect("Couldn't enable INTCN");
  rtc.enable_alarm1_interrupts().expect("Couldn't enable AIE");

  //display temperature, for kicks
  let temp = rtc.get_temperature().unwrap();
  println!("Temperature (C): {} ", temp);
  
  //force release i2c bus
  let _dev = rtc.destroy_ds3231();
}

/// Halt the Pi and reawaken when the number of minutes given have elapsed.
pub fn reawaken_in_minutes(minutes: u32) {
  set_minutes_delay_alarm(minutes);  
  safe_shutdown();
}

/// Halt the Pi and reawaken at the time given
pub fn reawaken_at_time(date_time: &chrono::DateTime<chrono::Utc>) {
  set_date_time_alarm(&date_time);
  safe_shutdown();
}

