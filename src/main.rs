use std::collections::{BTreeMap, HashSet};
use std::{error, string};

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(Debug, Default)]
struct Dragon {
    pub no: u32,
    pub name: String,
    pub amount: u32,
    pub prior: bool,
}

impl TryFrom<&str> for Dragon {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        let mut data: [&str; 3] = Default::default();
        data[2] = value.split(" ").last().unwrap();
        data[0] = value.rsplit(" ").last().unwrap();
        data[1] =value.strip_prefix(data[0]).unwrap().strip_suffix(data[2]).unwrap().trim();
        let dragon = Dragon {
            no: match data[0].strip_suffix(".").unwrap().parse() {
                Ok(n) => n,
                Err(e) => {
                    println!("error data is {:?},parse error: {:?}", data[0],e);
                    panic!("error parsing data");
                }
            },
            name: data[1].to_string(),
            
            amount: data[2].parse().unwrap(),
            prior: false,
        };
        Ok(dragon)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let file = File::open("src/list.txt").await?;
    let br = BufReader::new(file);
    let mut lines = br.lines();
    let mut investMap = BTreeMap::<u128, HashSet<&str>>::new();
    while let Some(line) = lines.next_line().await? {
        let dragon: Dragon = line.as_str().try_into().unwrap();
        println!("{:?}", dragon);
    }
    return Ok(());
}
