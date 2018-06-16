use tick::{Tick, TickSpan};

pub enum ReminderOrder {
    Before,
    After,
}

type ReminderCallback = fn(tick: Tick);

pub struct Reminder {
    time: Tick,
    start_time: Tick,
    repeat_time: TickSpan,
    callback: ReminderCallback,
}
