use std::io::Error;

pub fn set_time_by_offset(offset: i64) -> Result<(), Error> {
    if cfg!(target_arch = "mips") {
        
    }

    Ok(())
}