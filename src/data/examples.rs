use rec_rsys::models::Item;

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

pub fn items_examples() -> Vec<Item> {
    vec![
        create_example(2, vec![0.5, 0.3], None),
        create_example(8, vec![0.9, 0.3], None),
        create_example(7, vec![0.5, 0.3], None),
        create_example(9, vec![0.4, 0.3], None),
        create_example(6, vec![0.2, 0.3], None),
        create_example(5, vec![0.7, 0.3], None),
    ]
}
