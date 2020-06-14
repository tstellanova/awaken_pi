/*
Copyright (c) 2018 Todd Stellanova

LICENSE: See LICENSE file
*/

#![crate_type = "lib"]


use chrono::{Datelike, Timelike,  Utc, DateTime};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use linux_embedded_hal as hal;
use ds323x::{ Ds323x, Rtcc, Hours, DayAlarm1, Alarm1Matching };


/// Ensure the RTC is set to the same time as the system clock
pub fn set_rtc_date_time_to_system_time() {
    let dev = hal::I2cdev::new("/dev/i2c-1").expect("could not grab i2c-1");
    let mut rtc = Ds323x::new_ds3231(dev);
    let actual_time = Utc::now();
    let time_str = actual_time.format("%Y%m%d_%H%M%SZ").to_string();
    println!("old: {}", time_str);


    let d = NaiveDate::from_ymd(actual_time.year() , actual_time.month() , actual_time.day() );
    let t = NaiveTime::from_hms_milli(
	actual_time.hour(), 
	actual_time.minute() ,
	actual_time.second() ,
	0);
	
    let datetime = NaiveDateTime::new(d,t);

    /*
    let datetime = ds323x::NaiveDateTime{
                              year: actual_time.year() as u16,
                              month: actual_time.month() as u8,
                              day: actual_time.day() as u8,
                              weekday: actual_time.weekday() as u8,
                              hour: Hours::H24(actual_time.hour() as u8),
                              minute: actual_time.minute() as u8,
                              second: actual_time.second() as u8,
                   };
    */
    rtc.set_datetime(&datetime).expect("couldn't set_datetime");
  
    let dt = rtc.get_datetime().expect("Couldn't get the date time");

    println!("new: {:04}{:02}{:02}_{:02}{:02}{:02}Z", dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute(), dt.second());
  
    //force release i2c bus
    let _dev = rtc.destroy_ds3231();
}

/// Get the date and time according to the RTC
pub fn get_date_time() -> chrono::DateTime<chrono::Utc> {
  
  let dev = hal::I2cdev::new("/dev/i2c-1").expect("could not grab i2c-1");
  let mut rtc = Ds323x::new_ds3231(dev);
  let dt = rtc.get_datetime().expect("could not get time");
  
  let ndt: NaiveDateTime = NaiveDate::from_ymd(
    dt.year()  as i32,
    dt.month()  as u32,
    dt.day()  as u32 )
    .and_hms(
	dt.hour(),
      dt.minute() as u32,
      dt.second() as u32);
      
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
  
  let now_hours:u32 = dt.hour();
  let future_minutes:u32 = dt.minute() + minutes_delay as u32;
  let minutes = future_minutes % 60;
  let hours = now_hours  + (future_minutes / 60);
  // day does not matter, since we're using HoursMinutesAndSecondsMatch below
  let alarm1 = DayAlarm1 {
      day: 1, // unused
      hour: Hours::H24(hours as u8), 
      minute: minutes as u8,
      second: 1
  };
  
  //  Alarm should fire when hours, minutes, and seconds match
  rtc.set_alarm1_day(alarm1, Alarm1Matching::HoursMinutesAndSecondsMatch ).expect("Couldn't set alarm");
  rtc.use_int_sqw_output_as_interrupt().expect("Couldn't enable INTCN");
  rtc.enable_alarm1_interrupts().expect("Couldn't enable AIE");

  //display temperature, for kicks
  let temp = rtc.get_temperature().unwrap();
  println!("Temperature (C): {} ", temp);
  
  //force release i2c bus
  let _dev = rtc.destroy_ds3231();
}





