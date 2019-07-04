use slog::{debug, Logger};
use std::io::Error;
use time::{Duration, Timespec};

#[cfg(target_os = "linux")]
pub fn set_time_by_offset(offset: Duration, log: Logger) -> Result<(), Error> {
    if cfg!(target_os = "linux") {
        debug!(
            log,
            "[timekeeper] Fixing time in {}.{} seconds...",
            offset.num_seconds(),
            offset.num_nanoseconds().unwrap(),
        );
        let new = time::now().to_timespec();
        debug!(
            log,
            "[timekeeper] Current time is {}.{} seconds", new.sec, new.nsec
        );
        let timespec = get_timespec(new + offset);
        debug!(
            log,
            "[timekeeper] With offset, current time would be of {}.{} seconds", timespec.tv_sec, timespec.tv_nsec
        );
        unsafe { libc::clock_settime(libc::CLOCK_REALTIME, &timespec) };
        let new = time::now().to_timespec();
        debug!(
            log,
            "[timekeeper] Fixed time is {}.{} seconds", new.sec, new.nsec
        );
    } else {
        panic!("Operative System or Architecture not supported.");
    }

    Ok(())
}

#[cfg(all(target_arch = "mips", target_os = "linux", target_pointer_width = "32"))]
fn get_timespec(timespec: Timespec) -> libc::timespec {
    libc::timespec {
        tv_sec: timespec.sec as i32,
        tv_nsec: timespec.nsec as i32,
    }
}

#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn get_timespec(timespec: Timespec) -> libc::timespec {
    libc::timespec {
        tv_sec: timespec.sec,
        tv_nsec: timespec.nsec as i64,
    }
}

#[cfg(all(target_arch = "arm", target_os="linux", target_env="gnu"))]
fn get_timespec(timespec: Timespec) -> libc::timespec {
    libc::timespec {
        tv_sec: timespec.sec as i32,
        tv_nsec: timespec.nsec as i32,
    }
}
