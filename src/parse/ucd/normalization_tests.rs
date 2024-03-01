/// тест из UCD
#[derive(Clone)]
pub struct NormalizationTest
{
    pub part: String,
    pub description: String,
    pub line: usize,
    pub c1: String,
    pub c2: String,
    pub c3: String,
    pub c4: String,
    pub c5: String,
}

impl core::fmt::Debug for NormalizationTest
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        macro_rules! codes {
            ($c:expr) => {{
                let s: String = $c
                    .chars()
                    .map(|c| format!("{:04X} ", u32::from(c)))
                    .collect();
                s
            }};
        }

        write!(
            f,
            "\n\nPart: {}\nDescription: {}\nLine: {}\n\nC1: {}\nC2: {}\nC3: {}\nC4: {}\nC5: {}\n",
            self.part,
            self.description,
            self.line,
            codes!(self.c1),
            codes!(self.c2),
            codes!(self.c3),
            codes!(self.c4),
            codes!(self.c5)
        )
    }
}

lazy_static! {
    /// тесты нормализации из UCD
    pub static ref NORMALIZATION_TESTS: Vec<NormalizationTest> = normalization_tests();
}

const DATA: &str = include_str!("./../../../data/ucd 15.1.0/NormalizationTest.txt");

/// получить тест по номеру строки
pub fn get_normalization_test(i: usize) -> Option<&'static NormalizationTest>
{
    NORMALIZATION_TESTS.iter().find(|t| t.line == i)
}

/// разбор NormalizationTest.txt из UCD
fn normalization_tests() -> Vec<NormalizationTest>
{
    let mut result = vec![];
    let mut part = String::new();

    for (i, line) in DATA.lines().enumerate() {
        if line.starts_with('#') {
            continue;
        }

        if line.starts_with('@') {
            part = line.to_owned();
            continue;
        }

        let (codes, description) = line.split_once('#').unwrap();
        let codes: Vec<&str> = codes.split(';').collect();

        if codes.len() != 6 {
            panic!("{}: некорректное количество полей теста", i);
        }

        macro_rules! codes {
            ($str: expr) => {{
                $str.split_whitespace()
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|v| unsafe {
                        char::from_u32_unchecked(u32::from_str_radix(v, 16).unwrap())
                    })
                    .collect()
            }};
        }

        result.push(NormalizationTest {
            part: part.clone(),
            description: description.to_owned(),
            line: i + 1,
            c1: codes!(codes[0]),
            c2: codes!(codes[1]),
            c3: codes!(codes[2]),
            c4: codes!(codes[3]),
            c5: codes!(codes[4]),
        })
    }

    result
}
