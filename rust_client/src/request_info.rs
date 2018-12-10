
use std::cmp::Ordering;
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestInfo<'a> {
    pub request_type: &'a str,
    pub address: &'a str,
    pub coin_index: u32,
    pub coin_id: u32,
    pub nonce: u32,
    pub user_id: &'a str,
    pub rate: u32, 
    pub buy_amount: u32,
    pub sell_amount: u32,
    pub buy_crypto_type: &'a str,
    pub sell_crypto_type: &'a str
}

impl<'a> Ord for RequestInfo<'a> {
    fn cmp(&self, other: &RequestInfo) -> Ordering {
        if self.sell_crypto_type == "BTC" {
            self.rate.cmp(&other.rate)
        } else {
            other.rate.cmp(&self.rate)
        }
    }
}

impl<'a> PartialOrd for RequestInfo<'a> {
    fn partial_cmp(&self, other: &RequestInfo) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for RequestInfo<'a> {

}

impl<'a> PartialEq for RequestInfo<'a> {
    fn eq(&self, other: &RequestInfo) -> bool {
        self.rate == other.rate
    }
}