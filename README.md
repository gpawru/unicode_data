# парсинг данных из UCD, UCA, CLDR

## UCD:

* **UNICODE**: `HashMap<u32, Codepoint>` - UnicodeData.txt, информация о кодпоинтах Unicode
* **BLOCKS**: `HashMap<String, CodepointsBlock>` - Blocks.txt, блоки кодпоинтов (диапазоны)
* **QC_NFD**, **QC_NFKD**, **QC_NFC**, **QC_NFKC**: `Vec<char>` - DerivedNormalizationProps.txt, быстрые проверки нормализации
* **COMPOSITION_EXCLUSIONS**: `Vec<u32>` - CompositionExclusions.txt, исключения композиции
* **NORMALIZATION_TESTS**: `Vec<NormalizationTest>` - NormalizationTest.txt, тесты нормализации

## UCA:

* **DUCET_WEIGHTS**, **DUCET_IMPLICIT_WEIGHTS**: `Vec<WeightsEntry>` - allkeys.txt, DUCET (второй вариант - с вычисляемыми весами)
* **DUCET**, **DUCET_IMPLICIT**: `HashMap<u32, TrieNode>` - бор, корневые элементы - одиночные кодпоинты, дочерние элементы для каждого уровня - N + 1 кодпоинтов в последовательности
* **COLLATION_TEST_DUCET_NON_IGNORABLE**, **COLLATION_TEST_DUCET_SHIFTED**: `Vec<CollationTest>` - CollationTest, тесты сопоставлений

## CLDR:

* **CLDR_UND_WEIGHTS**, **CLDR_UND_IMPLICIT_WEIGHTS**: `Vec<WeightsEntry>` - allkeys_CLDR.txt, CLDR-версия DUCET
* **CLDR_UND**, **CLDR_UND_IMPLICIT**: `HashMap<u32, TrieNode>` - аналогично **DUCET** / **DUCET_IMPLICIT**
* **COLLATION_TEST_CLDR_NON_IGNORABLE**, **COLLATION_TEST_CLDR_SHIFTED**: `Vec<CollationTest>` - CollationTest, тесты сопоставлений

*также в библиотеке можно найти методы для получения вычисляемых весов: implicit_weights (общий случай), implicit_weights_tangut, implicit_weights_nushu, implicit_weights_khitan, implicit_weights_core_han, implicit_weights_other_han, implicit_weights_unassigned, применимые как для **DUCET** так и для **CLDR_UND**.*
