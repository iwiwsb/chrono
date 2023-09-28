use crate::NaiveDate;
use crate::NaiveTime;

#[derive(Debug)]
/// Contains a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 (UTC).
pub struct FileTime {
    low_date_time: u32,
    high_date_time: u32,
}

impl FileTime {
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<FileTime> {
        if !(1601..=i32::MAX).contains(&year) {
            return None;
        }
        if !(1..=12).contains(&month) {
            return None;
        }

        todo!()
    }

    pub const fn date(&self) -> NaiveDate {
        todo!()
    }

    pub const fn time(&self) -> NaiveTime {
        todo!()
    }
}

impl From<FileTime> for u64 {
    fn from(value: FileTime) -> Self {
        (value.high_date_time as u64) << 32 | value.low_date_time as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filetime_to_u64() {
        assert_eq!(
            u64::from(FileTime { low_date_time: 0, high_date_time: 0x8000_0000 }),
            0x8000_0000_0000_0000u64
        );
    }
}
