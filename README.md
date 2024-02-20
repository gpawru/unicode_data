# парсинг данных из UCD, UCA, CLDR

## UCD:

* **UNICODE**: `HashMap<u32, Codepoint>` - UnicodeData.txt, информация о кодпоинтах Unicode
* **BLOCKS**: `HashMap<String, CodepointsBlock>` - Blocks.txt, блоки кодпоинтов (диапазоны)
* **QC_NFD**, **QC_NFKD**, **QC_NFC**, **QC_NFKC**: `Vec<char>` - DerivedNormalizationProps.txt, быстрые проверки нормализации
* **COMPOSITION_EXCLUSIONS**: `Vec<u32>` - CompositionExclusions.txt, исключения композиции
* **NORMALIZATION_TESTS**: `Vec<NormalizationTest>` - NormalizationTest.txt, тесты нормализации

## UCA:

* **DUCET**, **DUCET_IMPLICIT**: `Vec<WeightsEntry>` - allkeys.txt, DUCET (второй вариант - с вычисляемыми весами)
* **COLLATION_TEST_DUCET_NON_IGNORABLE**, **COLLATION_TEST_DUCET_SHIFTED** - CollationTest, тесты сопоставлений

## CLDR:

* **CLDR_UND**, **CLDR_UND_IMPLICIT**: `Vec<WeightsEntry>` - allkeys_CLDR.txt, CLDR-версия DUCET
* **COLLATION_TEST_CLDR_NON_IGNORABLE**, **COLLATION_TEST_CLDR_SHIFTED** - CollationTest, тесты сопоставлений
