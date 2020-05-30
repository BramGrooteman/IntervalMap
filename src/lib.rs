use std::collections::BTreeMap;
use std::hash::Hash;

pub trait Min {
    fn minimum() -> Self;
}

impl Min for i32 {
    fn minimum() -> i32 {
        std::i32::MIN
    }
}

/*
Implements a mapping that encapsulates an interval meaning that we can insert a
start and end bound, and all values inbetween these bounds will be mapped to a value.
The keys therefore need to be Ord and Eq, whereas the values only have to be Eq.

Upon construction a initial value will be provided. This value will be the underlaying
`default` interval going from -Inf to Inf. All values that do not fall into another
interval will be captured by this default value.

The borders of the interval are [A, B) where B is not part of the interval.
```
imap: IntervalMap<i32, char> = IntervalMap::new(&'a');
imap.insert(&10, &20, &'v'); // Inserts 'v' between 10 and 20
imap.get(&15) == 'v'
```

The map is
*/
#[derive(Debug)]
pub struct IntervalMap<'a, K, V> {
    m_map: BTreeMap<K, &'a V>, // Keeping track of the mapping. Keys in order
}

impl<'a, K, V> IntervalMap<'a, K, V>
where
    K: Eq + Hash + Copy + Ord + Min,
    V: Eq,
{
    pub fn new(init_val: &'a V) -> Self {
        let mut m_map: BTreeMap<K, &V> = BTreeMap::new();
        m_map.insert(K::minimum(), init_val);
        return IntervalMap { m_map };
    }

    /* This was the exercise */
    pub fn insert(&mut self, begin_key: K, end_key: K, val: &'a V) {
        let mut before_val = self.get(&begin_key);
        let mut before_key = &begin_key;
        let mut to_delete: Vec<K> = vec![];

        let same_value = before_val == val;

        for elem in self.m_map.keys().into_iter() {
            if elem < &begin_key {
                continue;
            }
            if elem > &end_key {
                break;
            }

            if before_key != elem {
                to_delete.push(*elem);
            }

            before_key = elem;
            before_val = self.m_map.get(elem).unwrap();
        }

        for elem in to_delete {
            self.m_map.remove(&elem);
        }

        if !same_value {
            self.m_map.insert(begin_key, val);
        }
        if before_val != val {
            self.m_map.insert(end_key, before_val);
        }
    }

    // Always returns something
    pub fn get(&self, key: &K) -> &'a V {
        // Try to find the key
        let sorted_keys = self.m_map.keys().collect::<Vec<_>>();
        let idx = self.find_index(&sorted_keys, key);
        return self.m_map.get(sorted_keys.get(idx).unwrap()).unwrap();
    }

    fn find_index(&self, keys: &Vec<&K>, key: &K) -> usize {
        let mut low = 0;
        let mut high = (keys.len() - 1) as i32;
        while low <= high {
            let mid = low + ((high - low) / 2);
            if keys[mid as usize] < key {
                low = mid + 1
            } else if keys[mid as usize] > key {
                high = mid - 1
            } else {
                return mid as usize;
            }
        }

        if high < 0 {
            return 0;
        } else if low as usize > keys.len() - 1 {
            return keys.len() - 1;
        } else {
            return if low < high {
                low as usize
            } else {
                high as usize
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut test_map: IntervalMap<i32, char> = IntervalMap::new(&'a');
        assert_eq!(test_map.get(&10), &'a');

        test_map.insert(10, 20, &'b');
        assert_eq!(test_map.get(&11), &'b');
        assert_eq!(test_map.get(&20), &'a');

        test_map.insert(25, 30, &'c');
        assert_eq!(test_map.get(&10), &'b');
        assert_eq!(test_map.get(&20), &'a');
        assert_eq!(test_map.get(&26), &'c');
        assert_eq!(test_map.get(&30), &'a');
    }

    #[test]
    fn overlapping() {
        let mut test_map: IntervalMap<i32, char> = IntervalMap::new(&'a');
        test_map.insert(10, 20, &'b');
        test_map.insert(15, 25, &'c');

        assert_eq!(test_map.get(&12), &'b');
        assert_eq!(test_map.get(&15), &'c');
        assert_eq!(test_map.get(&20), &'c');
        assert_eq!(test_map.get(&25), &'a');
    }

    #[test]
    fn intertwined() {
        let mut test_map: IntervalMap<i32, char> = IntervalMap::new(&'a');
        test_map.insert(10, 20, &'b');
        test_map.insert(20, 30, &'c');

        assert_eq!(test_map.get(&10), &'b');
        assert_eq!(test_map.get(&20), &'c');
        assert_eq!(test_map.get(&25), &'c');
        assert_eq!(test_map.get(&30), &'a');
    }

    #[test]
    fn multiple_overlapping() {
        let mut test_map: IntervalMap<i32, char> = IntervalMap::new(&'a');
        test_map.insert(20, 25, &'b');
        test_map.insert(10, 25, &'c');
        test_map.insert(22, 50, &'c');
        assert_eq!(test_map.get(&5), &'a');
        assert_eq!(test_map.get(&10), &'c');
        assert_eq!(test_map.get(&20), &'c');
        assert_eq!(test_map.get(&51), &'a');
        for (k, v) in &test_map.m_map {
            println!("{} - {}", k, v);
        }
    }

    // fn canonical() {

    // }
}
