// generic-exercice.rs
use std::collections::HashMap;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let jeremy = Person {
        name: "jeremy".to_string(),
        age: 28,
    };
    let harry = Person {
        name: "harry".to_string(),
        age: 18,
    };

    let mut hm = HashMap::new();
    hm.insert(1, jeremy.clone());
    hm.insert(2, harry.clone());

    let mut bt = BTreeMap::new();
    bt.insert("A", jeremy.clone());
    bt.insert("B", harry.clone());
}
