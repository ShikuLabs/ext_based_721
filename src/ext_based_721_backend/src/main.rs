use ic_cdk::export::candid::{candid_method, Nat};
use ic_cdk::export::Principal;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use prop::PropMetadata;
mod module;
mod prop;
use crate::module::dip721;
use crate::module::token_identifier;
use crate::module::types::*;
use cap_sdk::IndefiniteEvent;

#[init]
#[candid_method(init)]
fn init(args: Option<InitArgs>) {
    dip721::dip721_init(args)
}

#[update]
#[candid_method(update)]
fn init_prop() -> Vec<prop::PropMetadata> {
    prop::init()
}


#[allow(non_snake_case)]
#[update]
#[candid_method(update)]
fn mintNFT(mint_request: MintRequest) -> TokenIndex {
    mint_internal(mint_request)
}

fn mint_internal(mint_request: MintRequest) -> TokenIndex {
    let token_id = dip721::new_token_id();
    let class = mint_request.class;
    let pid = ic_cdk::api::id();
    let cid = token_identifier::CanisterId(pid);
    let encode_idx = token_identifier::TokenIndex(token_id as u32);
    let encoded_token = token_identifier::encode_token_id(cid, encode_idx);

    let token_obj = token_identifier::decode_token_id(&encoded_token).unwrap();
    let arg_mint = Nat::from(token_obj.index.get_value());
    prop::add_token(&arg_mint, &encoded_token);

    let to = match mint_request.to {
        User::principal(pid) => pid,
        User::address(_aid) => Principal::anonymous(),
    };

    let properties = prop::with(|props| {
        props
            .iter()
            .filter(|p| *p.class() == class)
            .map(|p| {
                vec![
                    (
                        String::from("class"),
                        GeneralValue::TextContent(p.class().clone()),
                    ),
                    (
                        String::from("desc"),
                        GeneralValue::TextContent(p.desc().clone()),
                    ),
                    (
                        String::from("imageUri"),
                        GeneralValue::TextContent(p.image_uri().clone()),
                    ),
                ]
            })
            .next()
            .unwrap_or(vec![])
    });

    let res = dip721::dip721_mint(to, arg_mint, properties);
    res.unwrap().to_string().parse::<u32>().unwrap()
}

#[query]
#[candid_method(query)]
pub fn token_identifier(id: Nat) -> String {
    prop::tokens(&id)
}

#[query]
#[candid_method(query)]
pub fn metadata(token: token_identifier::TokenIdentifier) -> Option<TokenMetaDataExt> {
    let token_id = match token_identifier::decode_token_id(&token) {
        Ok(obj) => obj.index.get_value(),
        Err(e) => return None,
    };
    let metadata = dip721::dip721_token_metadata(Nat::from(token_id).to_owned());
    match metadata {
        Ok(data) => {
            let nest_value = GeneralValue::NestedContent(vec![
                (
                    "token_identifier".into(),
                    GeneralValue::NatContent(data.token_identifier),
                ),
                (
                    "is_burned".into(),
                    GeneralValue::BoolContent(data.is_burned),
                ),
                (
                    "properties".into(),
                    GeneralValue::NestedContent(data.properties),
                ),
                (
                    "minted_at".into(),
                    GeneralValue::Nat64Content(data.minted_at),
                ),
                ("minted_by".into(), GeneralValue::Principal(data.minted_by)),
                // (
                //     "transferred_at".into(),
                //     GeneralValue::Nat64Content(data.transferred_at.unwrap()),
                // ),
                // (
                //     "transferred_by".into(),
                //     GeneralValue::Principal(data.transferred_by.unwrap()),
                // ),
                // (
                //     "approved_at".into(),
                //     GeneralValue::Nat64Content(data.approved_at.unwrap()),
                // ),
                // (
                //     "approved_by".into(),
                //     GeneralValue::Principal(data.approved_by.unwrap()),
                // ),
                // (
                //     "burned_at".into(),
                //     GeneralValue::Nat64Content(data.burned_at.unwrap()),
                // ),
                // (
                //     "burned_by".into(),
                //     GeneralValue::Principal(data.burned_by.unwrap()),
                // ),
            ]);

            let value = match serde_json::to_vec(&nest_value) {
                Ok(v) => Some(v),
                Err(_) => None,
            };

            Some(TokenMetaDataExt::nonfungible({
                MetaDataNonFungibleDetails { metadata: value }
            }))
        }
        Err(_) => None,
    }
}

#[allow(non_snake_case)]
#[query]
#[candid_method(query)]
pub fn getTokens() -> Vec<(TokenIndex, TokenMetaDataExt)> {
    let mut res = Vec::new();
    let num = dip721::dip721_total_supply().to_string().parse::<u32>().unwrap();
    for item in 1..(num + 1) {
        match get_token_metadata_by_u32(item.to_owned()) {
            Ok(token) => {
                let token_metadata = 
                TokenMetaDataExt::nonfungible({
                    MetaDataNonFungibleDetails {
                        metadata: Some(token),
                    }
                });
                res.push((item.to_owned(), token_metadata.to_owned()));
            }
            Err(_) => {}
        };
    }
    res
 }

fn get_token_metadata_by_u32(id: u32) -> Result<Vec<u8>, CommonError> {
    let pid = ic_cdk::api::id();
    let cid = token_identifier::CanisterId(pid);
    let encode_idx = token_identifier::TokenIndex(id);
    let encoded_token = token_identifier::encode_token_id(cid, encode_idx);
    let metadata = dip721::dip721_token_metadata(Nat::from(id));
    let res = match metadata {
        Ok(data) => {
            let nest_value = GeneralValue::NestedContent(vec![
                (
                    "token_identifier".into(),
                    GeneralValue::NatContent(data.token_identifier),
                ),
                (
                    "is_burned".into(),
                    GeneralValue::BoolContent(data.is_burned),
                ),
                (
                    "properties".into(),
                    GeneralValue::NestedContent(data.properties),
                ),
                (
                    "minted_at".into(),
                    GeneralValue::Nat64Content(data.minted_at),
                ),
                ("minted_by".into(), GeneralValue::Principal(data.minted_by)),
            ]); 
            let value = match serde_json::to_vec(&nest_value) {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            value
        },
        Err(_) => return Err(CommonError::InvalidToken(encoded_token)),
    };
    Ok(res.unwrap())
}

#[query]
#[candid_method(query)]
fn supply() -> Result_2 {
    Result_2::ok(dip721::dip721_total_supply())
}


#[update]
#[candid_method(update)]
fn approve(approve_request: ApproveRequest) -> bool {
    let spender = approve_request.spender;
    let token = approve_request.token;
    let token_obj = token_identifier::decode_token_id(&token).unwrap();
    let token_index = Nat::from(token_obj.index.get_value());
    let approve_res = if let Some(value) = dip721::dip721_approve(spender, token_index).ok() {
        let _v = value;
        true
    } else {
        false
    };
    approve_res
}

#[update]
#[candid_method(update)]
fn burn(token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
    dip721::dip721_burn(token_identifier)
}

#[update]
#[candid_method(update)]
fn transfer(transfer_request: TransferRequest) -> TransferResponse {
    transfer_internal(transfer_request)
}

fn transfer_internal(transfer_request: TransferRequest) -> TransferResponse {
    let from = transfer_request.from;
    let to = transfer_request.to;
    let token = transfer_request.token;
    let from_pid = match from.clone() {
        User::principal(pid) => pid,
        User::address(_aid) => Principal::anonymous(),
    };
    let to_pid = match to {
        User::principal(pid) => pid,
        User::address(_aid) => Principal::anonymous(),
    };
    let token_obj = token_identifier::decode_token_id(&token).unwrap();
    let token_index = Nat::from(token_obj.index.get_value());

    match dip721::dip721_transfer_from(from_pid, to_pid, token_index) {
        Ok(resp) => TransferResponse::ok(resp),
        Err(NftError::UnauthorizedOwner) => TransferResponse::err(
            TransferResponseDetails::Unauthorized(User::aid(from.clone())),
        ),
        Err(_) => {
            TransferResponse::err(TransferResponseDetails::Other(String::from("Unkown Error")))
        }
    }
}

#[update]
#[candid_method(update)]
fn add(args: String) -> bool {
    let prop = match PropMetadata::new(&args) {
        Ok(prop) => prop,
        Err(_) => return false,
    };

    prop::with_mut(|props| props.push(prop));

    true
}

#[query]
#[candid_method(query)]
fn pending_transactions() -> Vec<IndefiniteEvent> {
    cap_sdk::pending_transactions()
}


#[update]
#[candid_method(update)]
fn batch_mint(
    mint_request: MintRequest,
    num: Option<u32>,
) -> Vec<TokenIndex> {
    let mut tids = vec![];

    if let Some(num) = num {
        for _i in 0..num {
            let tid = mint_internal(mint_request.clone());
            tids.push(tid)
        }
    };
    tids
}
#[update]
#[candid_method(update)]
fn batch_transfer(transfer_request: TransferRequest, num: Option<u32>) -> TransferResponse {
    if let Some(num) = num {
        for _i in 0..num {
            transfer_internal(transfer_request.clone());
        }
    };

    TransferResponse::ok(Nat::from(num.unwrap()))
}

// fn class(tid: TokenIdentifier) -> String {

// }

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    ic_cdk::export::candid::export_service!();
    __export_service()
}
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    std::print!("{}", export_candid());
}
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}
