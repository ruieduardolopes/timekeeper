use slog::{debug, Logger};
use std::io::Error;
use time::{Duration, Timespec};

#[cfg(target_os = "linux")]
pub fn set_time_by_offset(offset: Duration, dry_run: bool, log: Logger) -> Result<(), Error> {
    let mut new_clock: libc::timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if cfg!(target_os = "linux") {
        debug!(
            log,
            "[timekeeper] Fixing time in {} seconds and {} nanoseconds...",
            offset.num_seconds(),
            offset.num_nanoseconds().unwrap(),
        );
        let new = time::now().to_timespec();
        debug!(
            log,
            "[timekeeper] Current time is {} seconds and {} nanoseconds", new.sec, new.nsec
        );
        let timespec = get_timespec(new + offset);
        debug!(
            log,
            "[timekeeper] With offset, current time would be of {} seconds and {} nanoseconds",
            timespec.tv_sec,
            timespec.tv_nsec
        );
        if dry_run {
            new_clock = timespec;
        } else {
            unsafe { libc::clock_settime(libc::CLOCK_REALTIME, &timespec) };
        }
        let new = time::now().to_timespec();
        if dry_run {
            debug!(
                log,
                "[timekeeper] Fixed time is {} seconds and {} nanoseconds",
                new_clock.tv_sec,
                new_clock.tv_nsec
            )
        } else {
            debug!(
                log,
                "[timekeeper] Fixed time is {} seconds and {} nanoseconds", new.sec, new.nsec
            );
        }
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

#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
fn get_timespec(timespec: Timespec) -> libc::timespec {
    libc::timespec {
        tv_sec: timespec.sec as i32,
        tv_nsec: timespec.nsec as i32,
    }
}
