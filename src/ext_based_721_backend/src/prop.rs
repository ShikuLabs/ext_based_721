use std::cell::RefCell;
use ic_cdk::export::candid::{Nat};
use candid::{Deserialize, CandidType};
use serde::Serialize;
use crate::module::token_identifier;
use serde_json::Result;
use std::collections::HashMap;
pub static PROP_STR: &str =r###"
    [
        {"desc":"The basic element that makes up the world-hydrogen. General purpose in SHIKU games","image_uri": "https://storageapi.fleek.co/zimhook-team-bucket/Yumi/530x640.jpg","calss":"H"},
        {"desc":"The basic element that makes up the world-Helium. General purpose in SHIKU games","image_uri": "https://storageapi.fleek.co/zimhook-team-bucket/Yumi/530x640(2).jpg","calss":"He"},
        {"desc":"The basic element that makes up the world-Lithium. General purpose in SHIKU games","image_uri": "https://storageapi.fleek.co/zimhook-team-bucket/Yumi/530x640(3).jpg","calss":"Li"}
    ]
    "###;
    

#[derive(Deserialize, Serialize, Default, CandidType, Clone)]
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

    static ID2TOKEN :RefCell<HashMap<Nat, token_identifier::TokenIdentifier>> = RefCell::new(HashMap::new());

    static ID2PROP :RefCell<HashMap<Nat, PropMetadata>> = RefCell::new(HashMap::new()); 
}
pub fn with<T, F: FnOnce(&Vec<PropMetadata>) -> T>(f: F) -> T {
    PROPS.with(|props| f(&props.borrow()))
}

pub fn with_mut<T, F: FnOnce(&mut Vec<PropMetadata>) -> T>(f: F) -> T {
    PROPS.with(|props| f(&mut props.borrow_mut()))
}


pub fn prop_info() -> Vec<PropMetadata> {
    with(|prop| {
        prop.clone()
    }).to_vec()
}

pub fn restore_prop_info(prop_info: Vec<PropMetadata>) {
    with_mut(|prop_mut| {
        prop_info.iter().for_each(|prop| prop_mut.push(prop.to_owned()));
    });
}

fn init_prop_info(prop_vec: Vec<PropMetadata>) {
    prop_vec.iter().for_each(|prop| {
        with_mut(|prop_mut| {
            prop_mut.push(prop.clone());
        })
    });
}

pub fn init() -> Vec<PropMetadata>  {
    let prop_vec: Vec<PropMetadata> =serde_json::from_str(PROP_STR).expect("prop info init failed");
    init_prop_info(prop_vec);
    prop_info()
}

pub fn add_token(id: &Nat, prop: &String) {
    ID2TOKEN.with(|tokenmap| {
        tokenmap.borrow_mut().insert(id.to_owned(), prop.clone());
    })
}

// pub fn add_prop(id: &Nat, prop: &PropMetadata) {
//     ID2PROP.with(|propmap| {
//         propmap.borrow_mut().insert(id.to_owned(), prop.clone());
//     })
// }

pub fn tokens(id: &Nat) -> String {
    ID2TOKEN.with(|tokenmap|{
        match tokenmap.borrow().get(id) {
            Some(tokenid) => tokenid.to_owned(),
            None => "".into(),
        }
    })
}
