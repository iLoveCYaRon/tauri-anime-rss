
use crate::model::{RssChannel, RssItem};
use crate::Result;
use anyhow::{anyhow, Context};
use rss::Channel;
use std::{io::{BufRead, BufReader}, fs::File, env, path::PathBuf, vec};

pub(crate) fn get_channel_list() -> Result<Vec<RssChannel>> {
    let channel = read_file_2_channel()?;
    let rss_channel = RssChannel {
        id: "sample_mikan".to_owned(),
        title: channel.title,
        desc: channel.description,
        link: channel.link,
        status: crate::model::ChannelStatus::Succeed,
        item_count: channel.items.len() as u64,
    };
    Ok(vec![rss_channel])
}

pub(crate) fn get_channel_detail(channel_id: &str) -> Result<Vec<RssItem>> {
    let channel = read_file_2_channel()?;
    if channel_id != "sample_mikan" {
        return Err(anyhow!("channel_id: {} not found", channel_id))
    };
    let item_list = channel.items.into_iter()
        .map(|item| RssItem {
            id: item.guid.unwrap_or_default().value,
            title: item.title.unwrap_or_default(),
            desc: item.description.unwrap_or_default(),
            link: item.link.unwrap_or_default(),
            pub_date: item.pub_date.unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    Ok(item_list)
}

fn read_file_2_channel() -> Result<Channel> {
    let mut xml_path = env::current_dir()
        .context("Failed to get current directory")?;
    xml_path.push(PathBuf::from("assets/sample_mikan.rss"));
    println!("{}", xml_path.display());
    
    
    let file = File::open(xml_path)
        .context("Failed to open xml file")?;
    Channel::read_from(BufReader::new(file))
        .context("Failed to parse xml file")
}
#[cfg(test)]
mod test {
    use super::{get_channel_list, get_channel_detail};

    #[test]
    fn test_parse_local_file() {
        let cl = get_channel_list().unwrap().pop().unwrap();
        println!("{:?}", cl);

        let il = get_channel_detail("sample_mikan").unwrap();
        println!("{:?}", il.get(0));
    }
}