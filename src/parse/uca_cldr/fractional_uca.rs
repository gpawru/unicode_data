/// запись таблицы Fractional UCA
pub struct FractionalWeightsEntry
{
    /// последовательность кодов
    pub codes: Vec<u32>,
    /// веса
    pub weights: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    /// описание
    pub description: String,
    /// кейс L·, смотри https://www.unicode.org/reports/tr35/tr35-collation.html#Context_Sensitive_Mappings
    pub ce_case: bool,
}

lazy_static! {
    /// таблица c ремаппингом DUCET для использования с CLDR с весами переменной длины
    ///
    /// https://www.unicode.org/reports/tr35/tr35-collation.html#Root_Data_Files
    pub static ref FRACTIONAL_UCA_TABLE: Vec<FractionalWeightsEntry> = fractional_table();
}

const FRACTIONAL_UCA_SOURCE: &str = include_str!("./../../../data/cldr 44/FractionalUCA.txt");

fn fractional_table() -> Vec<FractionalWeightsEntry>
{
    let mut table = vec![];

    for line in FRACTIONAL_UCA_SOURCE.lines() {
        if line.is_empty() || line.starts_with(['[', '#']) {
            continue;
        }

        let (values, description) = line.split_once('#').unwrap();
        let (codes, weights) = values.split_once(';').unwrap();

        let ce_case = codes.contains('|');

        let codes = match ce_case {
            true => codes.replace('|', " "),
            false => codes.to_owned(),
        };

        let codes: Vec<u32> = codes
            .split_whitespace()
            .map(|v| {
                if !v.chars().all(|c| c.is_alphanumeric()) {
                    println!("{}", line);
                }

                assert!(v.chars().all(|c| c.is_alphanumeric()));

                u32::from_str_radix(v, 16).unwrap()
            })
            .collect();

        if weights.contains("U+") {
            // println!("{}", line);
            continue;
        }

        let mut weights: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)> = weights
            .trim()
            .trim_matches(['[', ']'])
            .split("][")
            .map(|weights| {
                let weights: Vec<Vec<u8>> = weights
                    .split(',')
                    .map(|v| {
                        v.trim()
                            .split_whitespace()
                            .map(|w| u8::from_str_radix(w, 16).unwrap())
                            .collect()
                    })
                    .collect();

                assert_eq!(weights.len(), 3);
                (weights[0].clone(), weights[1].clone(), weights[2].clone())
            })
            .collect();

        if weights.len() == 1
            && weights[0].0.is_empty()
            && weights[0].1.is_empty()
            && weights[0].2.is_empty()
        {
            weights.clear();
        }

        table.push(FractionalWeightsEntry {
            codes,
            weights,
            description: description.to_string(),
            ce_case,
        })
    }

    table
}
