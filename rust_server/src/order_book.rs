use std::collections::BinaryHeap;

// import from our own modules
use request_info::RequestInfo;

pub struct OrderBook<'a> {
    pub buy_crypto_type: &'a str,
    pub sell_crypto_type: &'a str,
    pub priority_queue: BinaryHeap<&'a RequestInfo<'a>>
}