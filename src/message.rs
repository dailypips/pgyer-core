use market::*;
use timer::Reminder;

pub enum Event {
    // Market Data Event
    Ask(Ask),
    Bid(Bid),
    Trade(Trade),
    Level2Update(Level2Update),
    Level2Snapshot(Level2Snapshot),
    Bar(Bar),
    News(News),

    // Virtual Timer Event
    Reminder(Reminder),
}
