use std::collections::HashMap;


pub struct MapQueryStore {
    pub data: HashMap<String, String>,
}

impl MapQueryStore {
    pub fn new() -> MapQueryStore {
        MapQueryStore { data: HashMap::new() }
    }
    
}

pub trait QueryStoreTrait {
    fn insert_item(&mut self, _key: String, _value: String);
    fn get_item_by_id(&self, _id: i32) -> Option<&String>;
}


impl QueryStoreTrait for MapQueryStore {
    fn get_item_by_id(&self, _id: i32) -> Option<&String> {
        self.data.get(&_id.to_string())
    }

    fn insert_item(&mut self, _key: String, _value: String) {
        self.data.insert(_key, _value);
    }
}
