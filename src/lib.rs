struct HashNode<K, V> {
    key: K,
    value: V,
}

struct HashMap<K, V> {
    nodes: Vec<HashNode<K, V>>,
}

impl<K, V> HashMap<K, V>
where
    K: std::cmp::PartialEq,
    V: Clone,
{
    pub fn new() -> HashMap<K, V> {
        HashMap { nodes: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        for (index, element) in self.nodes.iter().enumerate() {
            if element.key == key {
                let found = element.value.clone();

                self.nodes.remove(index);
                self.nodes.push(HashNode {
                    key: key,
                    value: value,
                });

                return Some(found);
            }
        }

        self.nodes.push(HashNode {
            key: key,
            value: value,
        });

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashmap_insert() {
        let mut map: HashMap<String, String> = HashMap::new();

        assert_eq!(map.insert("fruit".to_string(), "apple".to_string()), None);
        assert_eq!(
            map.insert("fruit".to_string(), "strawberry".to_string()),
            Some("apple".to_string())
        );
    }
}
