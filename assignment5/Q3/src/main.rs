use std::collections::{self, HashMap};
#[derive(Debug)]
struct TrieNode {
    chs: HashMap<char, TrieNode>,
    value: Option<i32>,
}
impl TrieNode {
    fn get_words(&self, collection: &mut String) -> Vec<(String, Option<i32>)> {
        let mut words: Vec<(String, i32)> = Vec::new();

        match self.value {
            Some(node_val) => {
                words.push((collection.clone(), node_val));

                for (key, trie_node) in &self.chs {
                    collection.push(key.clone());
                    let more_words = trie_node.get_words(collection);
                    for s in more_words {
                        words.push(s);
                    }
                    collection.pop();
                }
            }
            None => {
                for (key, trie_node) in &self.chs {
                    collection.push(key.clone());
                    let more_words = trie_node.get_words(collection);
                    for s in more_words {
                        words.push(s);
                    }
                    collection.pop();
                }
            }
        }
        

        words
    }
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}
impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode {
                chs: HashMap::new(),
                value: None,
            },
        }
    }

    pub fn add_string(&mut self, string: String, value: i32) {
        let mut current_node = &mut self.root;

        for c in string.chars() {
            current_node = current_node.chs
            .entry(c)
            .or_insert(TrieNode {
            chs: HashMap::new(),
            value: None,
            });
        }
        current_node.value = Some(value);
    }

    pub fn iter(&self) -> Vec<(char, Option<i32>)> {
        let mut collection = String::new();
        let lmao = self.root.get_words(&mut collection);
        lmao
    }

    pub fn traverse(&self) {
        
    }
    


}

fn main() {
    let mut trie = Trie::new();
    trie.add_string("B".to_string(), 1);
    trie.add_string("Bar".to_string(), 2);
    //println!("{:#?}", trie);
    trie.traverse();
}
