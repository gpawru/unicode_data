
const COLLATION_TEST_DUCET_NON_IGNORABLE_SOURCE: &str =
    include_str!("./../../../data/uca 17.0.0/CollationTest/CollationTest_NON_IGNORABLE.txt");
const COLLATION_TEST_DUCET_SHIFTED_SOURCE: &str =
    include_str!("./../../../data/uca 17.0.0/CollationTest/CollationTest_SHIFTED.txt");
const COLLATION_TEST_CLDR_NON_IGNORABLE_SOURCE: &str =
    include_str!("./../../../data/cldr 48/CollationTest/CollationTest_CLDR_NON_IGNORABLE.txt");
const COLLATION_TEST_CLDR_SHIFTED_SOURCE: &str =
    include_str!("./../../../data/cldr 48/CollationTest/CollationTest_CLDR_SHIFTED.txt");

/// тест сопоставлений
#[derive(Debug, Clone)]
pub struct CollationTest
{
    /// последовательность кодов
    pub codes: Vec<u32>,
    /// L1
    pub l1: Vec<u16>,
    /// L2
    pub l2: Vec<u16>,
    /// L3
    pub l3: Vec<u16>,
    /// L4 (в SHIFTED)
    pub l4: Vec<u16>,
    /// описание
    pub description: String,
}

impl CollationTest
{
    /// кодпоинты в виде строки
    pub fn as_string(&self) -> String
    {
        self.codes
            .iter()
            .map(|&c| unsafe { char::from_u32_unchecked(c) })
            .collect()
    }
}

macro_rules! collation_test {
    ($fn:ident, $pub:ident, $source:expr) => {
        lazy_static! {
            pub static ref $pub: Vec<CollationTest> = $fn();
        }

        fn $fn() -> Vec<CollationTest>
        {
            parse_collation_test($source)
        }
    };
}

collation_test!(
    collation_test_ducet_non_ignorable,
    COLLATION_TEST_DUCET_NON_IGNORABLE,
    COLLATION_TEST_DUCET_NON_IGNORABLE_SOURCE
);
collation_test!(
    collation_test_ducet_shifted,
    COLLATION_TEST_DUCET_SHIFTED,
    COLLATION_TEST_DUCET_SHIFTED_SOURCE
);
collation_test!(
    collation_test_cldr_non_ignorable,
    COLLATION_TEST_CLDR_NON_IGNORABLE,
    COLLATION_TEST_CLDR_NON_IGNORABLE_SOURCE
);
collation_test!(
    collation_test_cldr_shifted,
    COLLATION_TEST_CLDR_SHIFTED,
    COLLATION_TEST_CLDR_SHIFTED_SOURCE
);

fn parse_collation_test(source: &str) -> Vec<CollationTest>
{
    let mut result = vec![];

    source
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .for_each(|line| {
            let (codes, description) = line.split_once(';').unwrap();

            let (description, weights) = description.split_at(description.rfind('[').unwrap());
            let description = description.trim_matches(['\t', ' ', '#']).to_string();

            let codes: Vec<u32> = codes
                .split_whitespace()
                .map(|code| u32::from_str_radix(code, 16).unwrap())
                .collect();

            let mut weights: Vec<Vec<u16>> = weights
                .trim_matches(['[', ']'])
                .trim_end_matches('|')
                .split('|')
                .map(|weights| {
                    weights
                        .trim()
                        .split_whitespace()
                        .map(|weight| u16::from_str_radix(weight, 16).unwrap())
                        .collect()
                })
                .collect();

            assert!(weights.len() <= 4);

            (0 .. 4 - weights.len()).for_each(|_| weights.push(vec![]));

            result.push(CollationTest {
                codes,
                l1: weights[0].clone(),
                l2: weights[1].clone(),
                l3: weights[2].clone(),
                l4: weights[3].clone(),
                description,
            });
        });

    result
}
