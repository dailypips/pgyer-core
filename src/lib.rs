//#![feature(plugin)]
//#![plugin(phf_macros)]
//extern crate phf;

#[macro_use]
extern crate lazy_static;

extern crate bimap;
extern crate chrono;

extern crate futures;
extern crate mio;

extern crate penny; // iso 4217 currency crate

//pub mod currency;
pub mod dispatch;
pub mod market;
pub mod message;
pub mod tick;
pub mod timer;
