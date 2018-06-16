//! queue 提供 <sender, receiver>对，用于外部系统与核心通讯
//! protobuf？？protobuf 接收外部的信息，转换成内部的通讯
//! 该模块只负责内部的通讯
//! Future 提供了ctx, 包含了
//! 
//! 
//! 
//! REDESIGN
//! queue 发消息给 pipe，同时要通知 bus wakeup？？

// 队列在 push 第一个元素的时候需要*通知* pipe 更新优先级
// 该通知使pipe将让队列插入到优先级堆里

// create data channel , wrap receiver into AddQueue Message , send to mpsc control channel,
// when first element push into data channel, create UpdateQueue message & send to ctl channel
// Q: send to ctl channel WOULD NOT Wake up the thread, it's NEED EventLoop like mio
// 

use std::sync::mpsc;
use std::collections::BinaryHeap;

pub struct Context {
    loop: EventLoop,
    sender: mpsc::Sender<EventLoopMessage>,
    wakeup: Wakeup,
}

enum EventLoopMessage {
    AddQueue(AddQueue),
    UpdateQueue(UpdateQueue),
    //DeleteQueue(DeleteQueue), // maybe NOT use
    AddTimer(AddTimer),
    UpdateTimer(UpdateTimer),
    DeleteTimer(DeleteTimer)
}

enum PipeType {
    MarketData,
    Trading,
    Service,
    Other
}

pub struct Timer {

}

pub struct EventLoop {
    sender: mpsc::Sender<EventLoopMessage>,
    receiver: mpsc::Receiver<EventLoopMessage>,
    // pipe
    marketdata: BinaryHeap<mpsc::Receiver>,
    trading: BinaryHeap<mpsc::Receiver>,
    service: BinaryHeap<mpsc::Receiver>,
    other: BinaryHeap<mpsc::Receiver>,
    // timer
    local: BinaryHeap<Timer>,
    exchange:BinaryHeap<Timer>
}

pub enum RunMode {
    DEFAULT,
    RUN_ONCE,
    RUN
}

impl EventLoop {
    fn new() -> EventLoop {

    }

    fn run(mode: RunMode) -> Result<> {

    }


    fn new_queue() -> mpsc::Sender {

    }

    fn next() -> 

}