use std::cell::RefCell;

use candid::Deserialize;
use serde::Serialize;

use serde_json::Result;

#[derive(Deserialize, Serialize, Default)]
pub struct PropMetadata {
    calss: String,
    desc: String,
    image_uri: String,
}

impl PropMetadata {
    pub fn new(data: &String) -> Result<PropMetadata> {
        let prop: PropMetadata = serde_json::from_str(data)?;

        Ok(prop)
    }

    pub fn class(&self) -> &String {
        &self.calss
    }

    pub fn desc(&self) -> &String {
        &self.desc
    }

    pub fn image_uri(&self) -> &String {
        &self.image_uri
    }
}

thread_local! {
    static PROPS: RefCell<Vec<PropMetadata>> = RefCell::new(Vec::new());
}
pub fn with<T, F: FnOnce(&Vec<PropMetadata>) -> T>(f: F) -> T {
    PROPS.with(|props| f(&props.borrow()))
}

pub fn with_mut<T, F: FnOnce(&mut Vec<PropMetadata>) -> T>(f: F) -> T {
    PROPS.with(|props| f(&mut props.borrow_mut()))
}
