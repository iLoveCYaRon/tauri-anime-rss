use crate::model::Channel;
use crate::Result;
use anyhow::Context;
use rss::Channel as RssChannel;
use std::{env, fs::File, io::BufReader, path::PathBuf, vec};

pub(crate) fn get_channel_list() -> Result<Vec<Channel>> {
    let channel = read_file_2_channel()?;
    let rss_channel = Channel {
        id: "sample_mikan".to_owned(),
        title: channel.title,
        desc: channel.description,
        link: channel.link,
        status: crate::model::ChannelStatus::Succeed,
        item_count: channel.items.len() as u64,
    };
    Ok(vec![rss_channel])
}

fn read_file_2_channel() -> Result<RssChannel> {
    let mut xml_path = env::current_dir().context("Failed to get current directory")?;
    xml_path.push(PathBuf::from("assets/sample_mikan.rss"));
    println!("{}", xml_path.display());

    let file = File::open(xml_path).context("Failed to open xml file")?;
    RssChannel::read_from(BufReader::new(file)).context("Failed to parse xml file")
}

#[cfg(test)]
mod test {
    use super::get_channel_list;

    #[test]
    fn test_parse_local_file() {
        let cl = get_channel_list().unwrap().pop().unwrap();
        println!("{:?}", cl);

        // let il = get_feed_list("sample_mikan").unwrap();
        // println!("{:?}", il.get(0));
    }
}
