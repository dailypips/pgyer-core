//! Tick表示自EPOC以来流逝的micro seconds, 时区为 UTC
//! 最大的 Tick 表达为9999-12-31 23:59:59:999_999 UTC
//! 最小的 Tick 表达为1970-01-01 00:00:00 000_000 UTC

use chrono::prelude::*;
use std::default::Default;
use std::fmt;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Tick {
    pub time: u64, //utc
}

pub const MIN_TICK: Tick = Tick { time: 0 };
pub const MAX_TICK: Tick = Tick {
    time: 253402300799999999,
};

impl Tick {
    #[inline]
    fn to_tick(dt: &DateTime<Utc>) -> Tick {
        let secs = dt.timestamp() as u64;
        let msecs = dt.timestamp_subsec_micros() as u64;
        let stamp = secs * 1_000_000 + msecs;
        Tick { time: stamp }
    }

    #[inline]
    fn from_tick(tick: &Tick) -> DateTime<Utc> {
        let secs = (tick.time / 1_000_000) as i64;
        let msecs = (tick.time % 1_000_000) as u32;
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(secs, msecs * 1000), Utc)
    }

    #[inline]
    pub fn now() -> Tick {
        let utc: DateTime<Utc> = Utc::now();
        Tick::to_tick(&utc)
    }

    pub fn to_utc(&self) -> DateTime<Utc> {
        Tick::from_tick(self)
    }

    pub fn from_utc(dt: &DateTime<Utc>) -> Option<Tick> {
        let year = dt.year();
        if year >= 1970 && year <= 9999 {
            return Some(Tick::to_tick(dt));
        }
        None
    }
}

impl fmt::Display for Tick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_utc())
    }
}

impl Default for Tick {
    fn default() -> Tick {
        Tick::now()
    }
}
