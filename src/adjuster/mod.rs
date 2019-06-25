use std::io::Error;
use std::process::Command;
use time::Duration;
use libc::*;

pub fn set_time_by_offset(offset: Duration) -> Result<(), Error> {
    if cfg!(target_os = "linux") {
        let new = time::now().to_timespec() + offset;
        let timespec = libc::timespec {
            // TODO: fix this conditionally given an arch...
            tv_sec: new.sec ,
            tv_nsec: new.nsec as i64,
        };
        unsafe { libc::clock_settime(libc::CLOCK_REALTIME, &timespec) };
    } else {
        panic!("Operative System or Architecture not supported.");
    }

    Ok(())
}
