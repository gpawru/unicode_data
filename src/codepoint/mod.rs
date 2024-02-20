mod bidi_class;
mod bidi_mirrored;
mod canonical_combining_class;
mod decomposition;
mod general_category;
mod numeric_type;
mod simple_case_mapping;

pub use bidi_class::BidiClass;
pub use bidi_mirrored::BidiMirrored;
pub use canonical_combining_class::CanonicalCombiningClass;
pub use decomposition::Decomposition;
pub use decomposition::DecompositionTag;
pub use general_category::GeneralCategory;
pub use numeric_type::NumericType;
pub use simple_case_mapping::SimpleCaseMapping;

use crate::parse::is_composition_exclusion;
use crate::CodepointsBlock;

/// Кодпоинт Unicode
/// источник - UCD, UnicodeData.txt
#[derive(Debug, Clone)]
pub struct Codepoint
{
    /// код символа
    pub code: u32,
    /// название
    pub name: String,
    /// категория символа (general category)
    pub gc: GeneralCategory,
    /// класс канонического комбинирования (canonical combining class)
    pub ccc: CanonicalCombiningClass,
    /// класс направления (bidi class)
    pub bc: BidiClass,
    /// числовой тип
    pub numeric: NumericType,
    /// "зеркальный" символ двунаправленого текста (bidi mirrored)
    pub bidi_mirrored: BidiMirrored,
    /// соответствующая прописная буква
    pub simple_uppercase_mapping: SimpleCaseMapping,
    /// соответствующая строчная буква
    pub simple_lowercase_mapping: SimpleCaseMapping,
    /// соответствующая заглавная буква
    pub simple_titlecase_mapping: SimpleCaseMapping,
    /// тег декомпозиции
    pub decomposition_tag: Option<DecompositionTag>,
    /// декомпозиция
    pub decomposition: Vec<u32>,
    /// полная каноническая декомпозиция
    pub canonical_decomposition: Vec<u32>,
    /// полная декомпозиция совместимости
    pub compat_decomposition: Vec<u32>,
    // блок, к которому относится кодпоинт (Blocks.txt)
    pub block: Option<&'static CodepointsBlock>,
}

impl Codepoint
{
    /// является ли кодпоинт стартером
    #[inline]
    pub fn is_starter(&self) -> bool
    {
        self.ccc.is_starter()
    }

    /// кодпоинт не является стартером
    #[inline]
    pub fn is_nonstarter(&self) -> bool
    {
        self.ccc.is_nonstarter()
    }

    /// у кодпоинта - каноническая декомпозиция
    #[inline]
    pub fn has_canonical_decomposition(&self) -> bool
    {
        self.decomposition_tag.is_none() && !self.decomposition.is_empty()
    }

    /// у кодпоинта - декомпозиция совместимости
    #[inline]
    pub fn has_compat_decomposition(&self) -> bool
    {
        self.decomposition_tag.is_some() && !self.decomposition.is_empty()
    }

    /// является ли кодпоинт исключением композиции?
    #[inline]
    pub fn is_composition_exclusion(&self) -> bool
    {
        is_composition_exclusion(self.code)
    }
}

#[derive(Debug, PartialEq)]
pub enum PropertiesError
{
    UnknownPropertyValue,
}

impl From<core::num::ParseIntError> for PropertiesError
{
    fn from(_: core::num::ParseIntError) -> Self
    {
        Self::UnknownPropertyValue
    }
}
