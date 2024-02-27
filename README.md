# парсинг данных из UCD, UCA, CLDR

## UCD:

* **UNICODE**: `HashMap<u32, Codepoint>` - UnicodeData.txt, информация о кодпоинтах Unicode
* **BLOCKS**: `HashMap<String, CodepointsBlock>` - Blocks.txt, блоки кодпоинтов (диапазоны)
* **QC_NFD**, **QC_NFKD**, **QC_NFC**, **QC_NFKC**: `Vec<char>` - DerivedNormalizationProps.txt, быстрые проверки нормализации
* **COMPOSITION_EXCLUSIONS**: `Vec<u32>` - CompositionExclusions.txt, исключения композиции
* **NORMALIZATION_TESTS**: `Vec<NormalizationTest>` - NormalizationTest.txt, тесты нормализации

### нормализация:

* **NFD**, **NFKD**: `HashMap<u32, Vec<Codepoint>>` - полностью разложенная NFD / NFKD декомпозиция
* **NFC**, **NFKC**: `HashMap<u32, Vec<Codepoint>>` - прекомпозиция NFC / NFKC
* **COMPOSITION_PAIRS**: `HashMap<u32, HashMap<u32, Codepoint>>` - комбинируемые пары кодпоинтов
* **COMBINES_BACKWARDS**: `HashMap<u32, HashMap<u32, Codepoint>>` - кодпоинты, комбинируемые с предыдущими

## UCA:

* **DUCET**: `Vec<WeightsEntry>` - allkeys.txt, DUCET
* **DUCET_FILTERED_TRIE**: `HashMap<u32, TrieNode>` - DUCET в виде дерева, содержащая только NFD-кодпоинты
* **COLLATION_TEST_DUCET_NON_IGNORABLE**, **COLLATION_TEST_DUCET_SHIFTED**: `Vec<CollationTest>` - CollationTest, тесты сопоставлений

## CLDR:

* **CLDR_UND**: `Vec<WeightsEntry>` - allkeys_CLDR.txt, CLDR-версия DUCET
* **CLDR_FILTERED_TRIE**: `HashMap<u32, TrieNode>` - аналогично **DUCET_FILTERED_TRIE**
* **COLLATION_TEST_CLDR_NON_IGNORABLE**, **COLLATION_TEST_CLDR_SHIFTED**: `Vec<CollationTest>` - CollationTest, тесты сопоставлений
