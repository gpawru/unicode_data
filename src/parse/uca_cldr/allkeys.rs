use std::collections::HashMap;

use crate::QC_NFD;

/// запись таблицы DUCET / адаптированной таблицы DUCET для CLDR, полученная из allkeys.txt UCA / CLDR
#[derive(Debug, Clone)]
pub struct WeightsEntry
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

/// веса кодпоинта / последовательности кодпоинтов  
#[derive(Debug, Clone)]
pub struct TrieNode
{
    pub weights: &'static Vec<Weights>,
    pub children: Option<HashMap<u32, TrieNode>>,
}

impl TrieNode
{
    /// количество дочерних элементов
    pub fn children_len(&self) -> usize
    {
        match &self.children {
            Some(children) => children.len(),
            None => 0,
        }
    }
}

lazy_static! {
    /// DUCET в виде дерева, содержащая только NFD-кодпоинты
    pub static ref DUCET_FILTERED_TRIE: HashMap<u32, TrieNode> = weights_trie(&DUCET);
    /// таблица DUCET из allkeys.txt
    pub static ref DUCET: Vec<WeightsEntry> = allkeys(ALLKEYS_UCA);

    /// CLDR UND в виде дерева, содержащая только NFD-кодпоинты
    pub static ref CLDR_UND_FILTERED_TRIE: HashMap<u32, TrieNode> = weights_trie(&CLDR_UND);
    /// таблица DUCET из allkeys.txt, адаптированная для CLDR
    pub static ref CLDR_UND: Vec<WeightsEntry> = allkeys(ALLKEYS_CLDR);
}

const ALLKEYS_UCA: &str = include_str!("./../../../data/uca 15.1.0/allkeys.txt");
const ALLKEYS_CLDR: &str = include_str!("./../../../data/cldr 44/allkeys_CLDR.txt");

fn allkeys(source: &str) -> Vec<WeightsEntry>
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
        });
    }

    allkeys.sort_by_key(|e| e.codes.len());

    allkeys
}

#[inline]
fn weights_trie(table: &'static Vec<WeightsEntry>) -> HashMap<u32, TrieNode>
{
    let mut roots = HashMap::new();

    for entry in table.iter() {
        let codes = &entry.codes;

        // в последовательности есть ненормализованный кодпоинт - убираем
        if codes.iter().any(|&code| match QC_NFD.get(code as usize) {
            Some(&v) => v == 'N',
            None => false,
        }) {
            continue;
        }

        match codes.len() {
            1 => {
                roots.insert(
                    codes[0],
                    TrieNode {
                        weights: &entry.weights,
                        children: None,
                    },
                );
            }
            2 => {
                roots.entry(codes[0]).and_modify(|root| {
                    if root.children.is_none() {
                        root.children = Some(HashMap::new());
                    }

                    root.children.as_mut().unwrap().insert(
                        codes[1],
                        TrieNode {
                            weights: &entry.weights,
                            children: None,
                        },
                    );
                });
            }
            3 => {
                roots.entry(codes[0]).and_modify(|root| {
                    root.children
                        .as_mut()
                        .unwrap()
                        .entry(codes[1])
                        .and_modify(|second| {
                            if second.children.is_none() {
                                second.children = Some(HashMap::new());
                            }

                            second.children.as_mut().unwrap().insert(
                                codes[2],
                                TrieNode {
                                    weights: &entry.weights,
                                    children: None,
                                },
                            );
                        });
                });
            }
            _ => unreachable!(),
        }
    }

    roots
}
