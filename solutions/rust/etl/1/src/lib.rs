use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_tree = BTreeMap::new();
    for (point, letters) in h {
        for letter in letters {
            new_tree.insert(letter.to_ascii_lowercase(), *point);
        }
    }

    new_tree
}
