use std::collections::HashMap;
use std::ops::RangeInclusive;

lazy_static! {
    /// блоки кодпоинтов
    pub static ref CODEPOINTS_BLOCKS: HashMap<String, CodepointsBlock> = blocks();
}

#[derive(Debug, Clone)]
pub struct CodepointsBlock
{
    pub name: String,
    pub from: u32,
    pub to: u32,
}

impl CodepointsBlock
{
    pub fn range(&self) -> RangeInclusive<u32>
    {
        self.from ..= self.to
    }
}

/// найти блок по его названию
pub fn get_block_by_name(name: &str) -> Option<&'static CodepointsBlock>
{
    CODEPOINTS_BLOCKS.get(key(name).as_str())
}

/// найти блок символов, к которому относится кодпоинт
pub fn get_block_by_code(code: u32) -> Option<&'static CodepointsBlock>
{
    CODEPOINTS_BLOCKS
        .values()
        .into_iter()
        .find(|block| block.range().contains(&code))
}

const DATA: &str = include_str!("./../../../data/ucd 15.1.0/Blocks.txt");

/// получение блоков данных Blocks.txt из UCD
fn blocks() -> HashMap<String, CodepointsBlock>
{
    let mut map = HashMap::new();

    DATA.lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .for_each(|line| {
            let (range, name) = line.split_once(';').unwrap();
            let (from, to) = range.split_once("..").unwrap();

            let key = key(name);
            let name = name.trim().to_string();
            let from = u32::from_str_radix(from, 16).unwrap();
            let to = u32::from_str_radix(to, 16).unwrap();

            assert!(!map.contains_key(&key));

            map.insert(key, CodepointsBlock { name, from, to });
        });

    map
}

/// согласно Blocks.txt: When comparing block names, casing, whitespace, hyphens, and underbars are ignored.
/// см. https://www.unicode.org/reports/tr44
fn key(name: &str) -> String
{
    name.to_string()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}
