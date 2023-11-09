mod error;

use sonic_rs::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct GetChannelListResponse {
    pub(crate) channel_list: Vec<Channel>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct GetFeedListResponse {
    pub(crate) feed_list: Vec<Feed>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct CreateChannelResponse {
    pub(crate) channel_id: String,
}

// pub(crate) struct RefreshAllChannelRequest;

// pub(crate) struct RefreshAllChannelResponse;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct GetSettingResponse {
    pub(crate) setting: Setting,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct UpdateSettingResponse {
    pub(crate) setting: Setting,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Setting {
    pub(crate) network_proxy: NetworkProxy,
    pub(crate) qbit_config: QBitConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct QBitConfig {
    pub(crate) address: String,
    pub(crate) port: i32,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct NetworkProxy {
    pub(crate) proxy_type: ProxyType,
    pub(crate) address: String,
    pub(crate) port: i32,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) enum ProxyType {
    Https = 1,
    Socks5 = 2,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Channel {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) desc: String,
    pub(crate) link: String,
    pub(crate) status: ChannelStatus,
    pub(crate) item_count: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Feed {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) desc: String,
    pub(crate) link: String,
    pub(crate) pub_date: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct InvokeError {
    pub(crate) code: Option<i32>,
    pub(crate) msg: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) enum ChannelStatus {
    Init = 1,
    Succeed = 2,
    Loading = 3,
    Failed = 4,
}
