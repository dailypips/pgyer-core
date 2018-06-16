use tick::{Tick, TickSpan};

pub struct EventHead {
    source: u64,
    time: Tick,
}

pub struct MarketDataHead {
    source: u64,
    time: Tick,
    exchange_time: Tick,
    instrument: u64,
}

pub struct Reminder {
    source: u64,
    time: Tick,
    repeat_time: TickSpan,
}

pub struct Ask {
    head: MarketDataHead,
    price: f64,
    size: i64,
}

pub struct Bid {
    head: MarketDataHead,
    price: f64,
    size: i64,
}

pub struct Trade {
    head: MarketDataHead,
    price: f64,
    size: i64,
    direction: i8,
}

pub enum Level2Side {
    Bid,
    Ask,
}

pub enum Level2Action {
    New,
    Change,
    Delete,
    Reset,
}

pub struct Level2 {
    price: f64,
    size: i64,
    side: Level2Side,
    action: Level2Action,
    position: i32,
}

pub struct Level2Update {
    head: MarketDataHead,
    entries: Vec<Level2>,
}

pub struct Level2Snapshot {
    head: MarketDataHead,
    bids: Vec<Bid>,
    asks: Vec<Ask>,
}

pub enum BarType {
    Time = 1,
    Tick,
    Volume,
    Range,
    Session,
    Renko,
    Custom,
}

pub enum BarStatus {
    Incomplete,
    Complete,
    Open,
    High,
    Low,
    Close,
}
pub struct Bar {
    head: MarketDataHead,
    bar_type: BarType,
    size: u64,
    high: f64,
    low: f64,
    open: f64,
    close: f64,
    volume: u64,
    open_int: f64,
    count: u64,
    status: BarStatus,
}

pub enum NewsUrgency {
    Normal,
    Flash,
    Background,
}

pub struct News {
    head: MarketDataHead,
    urgency: String,
    url: String,
    headline: String,
    text: String,
}
