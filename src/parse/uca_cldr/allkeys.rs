use crate::get_block_by_name;

/// запись таблицы DUCET / адаптированной таблицы DUCET для CLDR, полученная из allkeys.txt UCA / CLDR
#[derive(Debug, Clone)]
pub struct WeightsEntry
{
    pub codes: Vec<u32>,
    pub weights: Vec<Weights>,
    pub description: String,
    pub is_implicit: bool,
}

/// веса для кодпоинта из DUCET, 3 уровня
#[derive(Debug, Clone, Copy)]
pub struct Weights
{
    pub l1: u16,
    pub l2: u16,
    pub l3: u16,
    pub is_variable: bool,
}

lazy_static! {
    /// таблица DUCET
    pub static ref DUCET: Vec<WeightsEntry> = allkeys(ALLKEYS_UCA, false);
    pub static ref DUCET_IMPLICIT: Vec<WeightsEntry> = allkeys(ALLKEYS_UCA, true);

    /// таблица DUCET, адаптированная для CLDR
    pub static ref CLDR_UND: Vec<WeightsEntry> = allkeys(ALLKEYS_CLDR, false);
    pub static ref CLDR_UND_IMPLICIT: Vec<WeightsEntry> = allkeys(ALLKEYS_CLDR, true);
}

const ALLKEYS_UCA: &str = include_str!("./../../../data/uca 15.1.0/allkeys.txt");
const ALLKEYS_CLDR: &str = include_str!("./../../../data/cldr 44/allkeys_CLDR.txt");

fn allkeys(source: &str, include_implicit_weights: bool) -> Vec<WeightsEntry>
{
    let mut allkeys = vec![];

    for line in source.lines() {
        if line.starts_with('#') || line.is_empty() || line.starts_with("@version") {
            continue;
        }

        if line.starts_with("@implicitweights") {
            continue;
        }

        let (values, description) = line.split_once(" # ").unwrap();
        let (codes, weights) = values.split_once(" ; ").unwrap();

        let description = description.to_string();

        let codes: Vec<u32> = codes
            .trim()
            .split(' ')
            .map(|v| u32::from_str_radix(v, 16).unwrap())
            .collect();

        let weights: Vec<Weights> = weights
            .trim_matches([' ', '[', ']'])
            .split("][")
            .map(|str| {
                let is_variable = str.chars().collect::<Vec<char>>()[0] == '*';
                let weights = str
                    .trim_matches(['.', '*'])
                    .split('.')
                    .map(|w| {
                        assert!(!w.is_empty());
                        assert!(w.chars().all(|c| c.is_ascii_alphanumeric()));

                        u16::from_str_radix(w, 16).unwrap()
                    })
                    .collect::<Vec<u16>>();

                assert_eq!(weights.len(), 3);

                Weights {
                    l1: weights[0],
                    l2: weights[1],
                    l3: weights[2],
                    is_variable,
                }
            })
            .collect();

        allkeys.push(WeightsEntry {
            codes,
            weights,
            description,
            is_implicit: false,
        });
    }

    if !include_implicit_weights {
        return allkeys;
    }

    // добавим вычисляемые значения
    // см. https://www.unicode.org/reports/tr10/tr10-49.html#Implicit_Weights

    // примечание:
    // UCD, Blocks.txt: Tangut Supplement: U+18D00 ..= U+18D7F.
    // UCA, allkeys.txt: Tangut Supplement: U+18D00 ..= U+18D8F, что является очевидной ошибкой.
    //
    // кроме того, на текущий момент (15.1.0) последний заданный кодпоинт, принадлежащий этому блоку - U+18D08.
    // так как за блоком Tangut Supplement следуют неназначенные кодпоинты - не имеет критического значения,
    // по какой формуле для этих кодпоинтов будут рассчитываться веса.

    macro_rules! block {
        ($name: expr) => {
            get_block_by_name($name).unwrap().range()
        };
        ($name: expr, assigned) => {
            get_block_by_name($name)
                .unwrap()
                .range()
                .filter(|c| crate::UNICODE.contains_key(c))
        };
    }

    block!("Tangut", assigned).for_each(|c| allkeys.push(implicit_weights_tangut(c)));
    block!("Tangut Components", assigned).for_each(|c| allkeys.push(implicit_weights_tangut(c)));
    block!("Tangut Supplement", assigned).for_each(|c| allkeys.push(implicit_weights_tangut(c)));
    block!("Nushu", assigned).for_each(|c| allkeys.push(implicit_weights_nushu(c)));
    block!("Khitan Small Script", assigned).for_each(|c| allkeys.push(implicit_weights_khitan(c)));

    // в allkeys.txt не упомянуты @implicitweights Han Unified Ideographs
    //
    // случай Core Han Unified Ideographs:
    //  - отсутствуют веса для блока CJK Unified Ideographs,
    //  - есть веса для Unified_Ideograph=True AND Block=CJK_Compatibility_Ideographs

    block!("CJK Unified Ideographs").for_each(|c| allkeys.push(implicit_weights_core_han(c)));

    // случай All other Han Unified Ideographs: веса отсутствуют. данный кейс включает в себя
    // все кодпоинты CJK Unified Ideographs Extension A ..= H

    ('A' ..= 'H').for_each(|c| {
        let block = format!("CJK Unified Ideographs Extension {}", c);
        block!(&block).for_each(|c| allkeys.push(implicit_weights_other_han(c)));
    });

    allkeys
}

fn implicit_weights(code: u32, aaaa: u16, bbbb: u16, description: &str) -> WeightsEntry
{
    WeightsEntry {
        codes: vec![code],
        weights: vec![
            Weights {
                l1: aaaa,
                l2: 0x0020,
                l3: 0x0002,
                is_variable: false,
            },
            Weights {
                l1: bbbb,
                l2: 0,
                l3: 0,
                is_variable: false,
            },
        ],
        description: description.to_owned(),
        is_implicit: true,
    }
}

pub fn implicit_weights_tangut(code: u32) -> WeightsEntry
{
    implicit_weights(
        code,
        0xFB00,
        ((code - 0x17000) | 0x8000) as u16,
        "Tangut codepoint",
    )
}

pub fn implicit_weights_nushu(code: u32) -> WeightsEntry
{
    implicit_weights(
        code,
        0xFB01,
        ((code - 0x1B170) | 0x8000) as u16,
        "Nushu codepoint",
    )
}

pub fn implicit_weights_khitan(code: u32) -> WeightsEntry
{
    implicit_weights(
        code,
        0xFB02,
        ((code - 0x18B00) | 0x8000) as u16,
        "Khitan codepoint",
    )
}

pub fn implicit_weights_core_han(code: u32) -> WeightsEntry
{
    implicit_weights(
        code,
        (0xFB40 + (code >> 15)) as u16,
        ((code & 0x7FFF) | 0x8000) as u16,
        "Core Han Unified Ideograph",
    )
}

pub fn implicit_weights_other_han(code: u32) -> WeightsEntry
{
    implicit_weights(
        code,
        (0xFB80 + (code >> 15)) as u16,
        ((code & 0x7FFF) | 0x8000) as u16,
        "Other Han Unified Ideograph",
    )
}

pub fn implicit_weights_unassigned(code: u32) -> WeightsEntry
{
    implicit_weights(
        code,
        (0xFBC0 + (code >> 15)) as u16,
        ((code & 0x7FFF) | 0x8000) as u16,
        "Unassigned",
    )
}
