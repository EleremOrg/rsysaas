use rec_rsys::models::Item;
use std::collections::HashMap;

pub fn create_example(id: u32, values: Vec<f32>, result: Option<f32>) -> Item {
    Item {
        id,
        values,
        result: match result {
            Some(value) => value,
            None => f32::NAN,
        },
    }
}

pub fn items_examples() -> HashMap<u32, Item> {
    let mut h = HashMap::new();
    h.insert(2, create_example(2, vec![0.5, 0.3], None));
    h.insert(8, create_example(8, vec![0.9, 0.3], None));
    h.insert(7, create_example(7, vec![0.5, 0.3], None));
    h.insert(9, create_example(9, vec![0.4, 0.3], None));
    h.insert(6, create_example(6, vec![0.2, 0.3], None));
    h.insert(5, create_example(5, vec![0.7, 0.3], None));
    h
}
