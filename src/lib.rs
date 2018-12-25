/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

#![crate_type = "lib"]





extern crate chrono;
use chrono::{Datelike, Timelike, Utc, DateTime};

extern crate runas;


extern crate linux_embedded_hal as hal;

extern crate ds323x;
use ds323x::{ Ds323x, Hours, DayAlarm1, Alarm1Matching };

/// Get the date and time according to the RTC
pub fn get_date_time() -> chrono::DateTime<chrono::Utc> {
  use chrono::{NaiveDate, NaiveDateTime};
  
  let dev = hal::I2cdev::new("/dev/i2c-1").expect("could not grab i2c-1");
  let mut rtc = Ds323x::new_ds3231(dev);

  let dt = rtc.get_datetime().expect("could not get time");
  
  //TODO more elegant variant conversion?
  let mut hours:u8 = 0;
  match dt.hour {
    Hours::H24(h) =>  {
      hours = h;
    },
    _ => {}
    
  }
  
  let ndt: NaiveDateTime = NaiveDate::from_ymd(
    dt.year  as i32,
    dt.month  as u32,
    dt.day  as u32 )
    .and_hms(
      hours as u32,
      dt.minute as u32,
      dt.second as u32);
      
  let cdt = DateTime::<Utc>::from_utc(ndt, Utc);
  let _dev = rtc.destroy_ds3231();
  
  cdt
}

/// Halt the Pi and reawaken when the number of minutes given have elapsed.
pub fn reawaken_in_minutes(minutes: u8) {
  set_minutes_delay_alarm(minutes);  
  safe_shutdown();
}


/// Shut down the system, entering HALT mode on raspberry pi
fn safe_shutdown() {
  use runas::Command;
  
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

/// Check whether the RTC is set to a reasonable time and, if not, set the time
pub fn check_and_reset_datetime() {
  let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
  let mut rtc = Ds323x::new_ds3231(dev);
  
  // Verify that the RTC is set to some reasonable time
  let cur_date = rtc.get_datetime().expect("Couldn't get the date time");
  if cur_date.year < 2018 {
    println!("bogus cur_date: reset!");
    let actual_time = Utc::now();

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
  
  //force release i2c bus
  let _dev = rtc.destroy_ds3231();
}

/// Tell the RTC to set an alarm by delay from the current time
fn set_minutes_delay_alarm(minutes_delay: u8) {
  let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
  let mut rtc = Ds323x::new_ds3231(dev);
  
  let dt = rtc.get_datetime().expect("could not get time");
  
  // The INT/SQW output will be latched low if the alarm has already fired: clear it
  if rtc.has_alarm1_matched().expect("Couldn't check alarm1") {
    println!("Alarm already fired!");
    rtc.clear_alarm1_matched_flag().expect("couldn't clear alarm1 flag");
  }
  
  let now_minutes: u32 = dt.minute as u32;
  
  //TODO more elegant variant conversion?
  let mut now_hours:u32 = 0;
  match dt.hour {
    Hours::H24(h) =>  {
      now_hours = h as u32;
    },
    _ => {}
  }
  
  let minutes = (now_minutes + (minutes_delay as u32)) % 60;
  let hours = now_hours + ((now_minutes + (minutes_delay as u32)) / 60);
  // day do not matter, since we're using HoursMinutesAndSecondsMatch below
  let alarm1 = DayAlarm1 {
      day: 1, // unused
      hour: Hours::H24(hours as u8), 
      minute: minutes as u8,
      second: 1
  };
  
  //  Alarm should fire when minutes and seconds match
  rtc.set_alarm1_day(alarm1, Alarm1Matching::HoursMinutesAndSecondsMatch ).expect("Couldn't set alarm");
  rtc.use_int_sqw_output_as_interrupt().expect("Couldn't enable INTCN");
  rtc.enable_alarm1_interrupts().expect("Couldn't enable AIE");

  //display temperature, for kicks
  let temp = rtc.get_temperature().unwrap();
  println!("Temperature (C): {} ", temp);
  
  //force release i2c bus
  let _dev = rtc.destroy_ds3231();
}





