use crate::{model::Feed, Result};
use anyhow::{anyhow, Context};
use rss::Channel as RssChannel;
use std::{env, fs::File, io::BufReader, path::PathBuf};

pub(crate) fn get_feed_list(channel_id: &str) -> Result<Vec<Feed>> {
    let channel = read_file_2_channel()?;
    if channel_id != "sample_mikan" {
        return Err(anyhow!("channel_id: {} not found", channel_id));
    };
    let item_list = channel
        .items
        .into_iter()
        .map(|item| Feed {
            id: item.guid.unwrap_or_default().value,
            title: item.title.unwrap_or_default(),
            desc: item.description.unwrap_or_default(),
            link: item.link.unwrap_or_default(),
            pub_date: item.pub_date.unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    Ok(item_list)
}

fn read_file_2_channel() -> Result<RssChannel> {
    let mut xml_path = env::current_dir().context("Failed to get current directory")?;
    xml_path.push(PathBuf::from("assets/sample_mikan.rss"));
    println!("{}", xml_path.display());

    let file = File::open(xml_path).context("Failed to open xml file")?;
    RssChannel::read_from(BufReader::new(file)).context("Failed to parse xml file")
}
