use std::collections::HashMap;

use crate::parse::UNICODE;

use super::PropertiesError;

lazy_static! {
    /// сжатые значения CCC
    pub static ref COMPRESED_CCC: HashMap<u8, u8> = compressed_ccc();
}

/// класс канонического комбинирования (Canonical Combining Class, CCC)
/// берется из UCD: третья колонка UnicodeData.txt
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanonicalCombiningClass(u8);

#[allow(non_upper_case_globals)]
impl CanonicalCombiningClass
{
    pub const NotReordered: Self = Self(0);
    pub const Overlay: Self = Self(1);
    pub const HanReading: Self = Self(6);
    pub const Nukta: Self = Self(7);
    pub const KanaVoicing: Self = Self(8);
    pub const Virama: Self = Self(9);
    pub const AttachedBelowLeft: Self = Self(200);
    pub const AttachedBelow: Self = Self(202);
    pub const AttachedAbove: Self = Self(214);
    pub const AttachedAboveRight: Self = Self(216);
    pub const BelowLeft: Self = Self(218);
    pub const Below: Self = Self(220);
    pub const BelowRight: Self = Self(222);
    pub const Left: Self = Self(224);
    pub const Right: Self = Self(226);
    pub const AboveLeft: Self = Self(228);
    pub const Above: Self = Self(230);
    pub const AboveRight: Self = Self(232);
    pub const DoubleBelow: Self = Self(233);
    pub const DoubleAbove: Self = Self(234);
    pub const IotaSubscript: Self = Self(240);
}

impl CanonicalCombiningClass
{
    /// стартер?
    #[inline]
    pub fn is_starter(&self) -> bool
    {
        self.0 == 0
    }

    /// нестартер?
    #[inline]
    pub fn is_nonstarter(&self) -> bool
    {
        self.0 != 0
    }

    /// в виде u8
    #[inline]
    pub fn u8(&self) -> u8
    {
        self.0
    }

    /// в сжатом до 6 бит виде - берём все CCC, используемые в UCD, сортируем их по возрастанию,
    /// возвращаем индекс элемента этого массива
    #[inline]
    pub fn compressed(&self) -> u8
    {
        COMPRESED_CCC[&self.0]
    }
}

macro_rules! from_into {
    ($($type:ty),+) => {
        $(
            impl From<$type> for CanonicalCombiningClass
            {
                #[inline]
                fn from(value: $type) -> Self
                {
                    Self(value as u8)
                }
            }

            impl From<CanonicalCombiningClass> for $type
            {
                #[inline]
                fn from(value: CanonicalCombiningClass) -> Self
                {
                    value.0 as $type
                }
            }
        )+
    }
}

from_into!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl TryFrom<&str> for CanonicalCombiningClass
{
    type Error = PropertiesError;

    fn try_from(value: &str) -> Result<Self, Self::Error>
    {
        Ok(Self::from(value.parse::<u8>()?))
    }
}

/// хешмап сжатых значений CCC
fn compressed_ccc() -> HashMap<u8, u8>
{
    let mut ccc_list = vec![];

    for codepoint in UNICODE.values() {
        let ccc = u8::from(codepoint.ccc);

        if !ccc_list.contains(&ccc) {
            ccc_list.push(ccc);
        }
    }

    ccc_list.sort();

    let mut result = HashMap::new();

    ccc_list.iter().enumerate().for_each(|(i, c)| {
        result.insert(*c, i as u8);
    });

    result
}
