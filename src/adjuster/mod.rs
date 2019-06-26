use libc::*;
use slog::{debug, Logger};
use std::io::Error;
use std::process::Command;
use time::Duration;

pub fn set_time_by_offset(offset: Duration, log: Logger) -> Result<(), Error> {
    if cfg!(target_os = "linux") {
        debug!(log, "[timekeeper] Fixing time in {} seconds...", offset.num_seconds());
        let new = time::now().to_timespec() + offset;
        debug!(log, "[timekeeper] Current time is {}.{} seconds", new.sec, new.nsec);
        let timespec = libc::timespec {
            // TODO: fix this conditionally given an arch...
            tv_sec: new.sec,
            tv_nsec: new.nsec as i64,
        };
        unsafe { libc::clock_settime(libc::CLOCK_REALTIME, &timespec) };
        let new = time::now().to_timespec();
        debug!(log, "[timekeeper] Fixed time is {}.{} seconds", new.sec, new.nsec);
    } else {
        panic!("Operative System or Architecture not supported.");
    }

    Ok(())
}
