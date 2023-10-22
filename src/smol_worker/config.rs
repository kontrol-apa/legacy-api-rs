use serde_derive::{Deserialize, Serialize};

#[derive(Debug,Deserialize, Serialize)]
pub struct Config {
    api_url: String,
    pub collection_address: String,
    graph_url: String,
    max_size: MaxSize,
    storage: Storage,
}
#[derive(Debug,Deserialize, Serialize)]
pub struct MaxSize {
    bids: u32,
    asks: u32,
    show_room: u32,
    largest_sales: u32,
    recent_sales: u32,
}
#[derive(Debug,Deserialize, Serialize)]
pub struct Storage {
    ask_key: String,
    bid_key: String,
    stats_key: String,
    show_room_key: String,
    recent_sales_key: String,
    largest_sales_key: String,
    collection_key: String,
}