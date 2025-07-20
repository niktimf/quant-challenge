use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BybitResponse<ResponseResult> {
    pub result: ResponseResult,
    pub ret_code: i32,
    pub ret_msg: String,
    pub time: i64,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookData {
    #[serde(rename = "a")]
    pub asks: Vec<(String, String)>,
    #[serde(rename = "b")]
    pub bids: Vec<(String, String)>,
    #[serde(rename = "s")]
    pub symbol: String,
    // #[serde(rename = "ts")]
    // pub timestamp: u64,
    // #[serde(rename = "u")]
    // pub update_id: u64,
    // #[serde(rename = "seq")]
    // pub sequence: Option<u64>,
    // #[serde(rename = "cts")]
    // pub cross_sequence: Option<u64>,
}

pub enum Category {
    Spot,
    Linear,
    Inverse,
    Option,
}
