struct HashNode<K, V> {
    key: K,
    value: V,
}

pub struct HashMap<K, V> {
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

    pub fn get(&self, key: K) -> Option<V> {
        for element in self.nodes.iter() {
            if element.key == key {
                return Some(element.value.clone());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_insert() {
        let mut map: HashMap<&str, &str> = HashMap::new();

        assert_eq!(map.insert("fruit", "apple"), None);
        assert_eq!(
            map.insert("fruit", "strawberry"),
            Some("apple")
        );
    }

    #[test]
    fn test_hashmap_get() {
        let mut map: HashMap<&str, &str> = HashMap::new();

        map.insert("fruit", "apple");
        map.insert("car", "sedan");

        assert_eq!(map.get("missing"), None);

        assert_eq!(map.get("fruit"), Some("apple"));
        assert_eq!(map.get("car"), Some("sedan"));
    }
}
