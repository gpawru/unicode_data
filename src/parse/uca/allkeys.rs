/// запись таблицы DUCET, полученная из allkeys.txt UCA
pub struct DucetEntry
{
    pub codes: Vec<u32>,
    pub weights: Vec<Weights>,
    pub description: String,
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
    /// не включает в себя вычисляемые веса:
    ///
    /// @implicitweights 17000..18AFF; FB00 # Tangut and Tangut Components
    /// @implicitweights 18D00..18D8F; FB00 # Tangut Supplement
    /// @implicitweights 1B170..1B2FF; FB01 # Nushu
    /// @implicitweights 18B00..18CFF; FB02 # Khitan Small Script
    ///
    /// https://www.unicode.org/reports/tr10/#Implicit_Weights
    pub static ref DUCET: Vec<DucetEntry> = allkeys();
}

const DATA: &str = include_str!("./../../../data/uca 15.1.0/allkeys.txt");

fn allkeys() -> Vec<DucetEntry>
{
    let mut ducet = vec![];

    for line in DATA.lines() {
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
                    .map(|w| u16::from_str_radix(w, 16).unwrap())
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

        ducet.push(DucetEntry {
            codes,
            weights,
            description,
        });
    }

    ducet
}
