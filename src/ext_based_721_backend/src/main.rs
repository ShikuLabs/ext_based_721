use ic_cdk::export::candid::{candid_method, Nat};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use ic_cdk::export::Principal;
mod module;
use crate::module::token_identifier;
use crate::module::types::*;
use crate::module::dip721;
#[init]
#[candid_method(init)]
fn init(args: Option<InitArgs>) {
    dip721::dip721_init(args)
    
}


#[update]
#[candid_method(update)]
fn mintNFT(mint_request: MintRequest) -> TokenIndex {
    let token_id = dip721::new_token_id();

    let pid = ic_cdk::api::id();
    let cid = token_identifier::CanisterId(pid);
    let encode_idx = token_identifier::TokenIndex(token_id as u32);
    let encoded_token = token_identifier::encode_token_id(cid, encode_idx);

    let token_obj = token_identifier::decode_token_id(&encoded_token).unwrap();
    let arg_mint = Nat::from(token_obj.index.get_value());


    let to = match mint_request.to {
        User::principal(pid) => pid,
        User::address(_aid) => Principal::anonymous(),
    };

    let metadata = match mint_request.metadata{
        Some(data) => GeneralValue::BlobContent(data),
        None => GeneralValue::BlobContent(vec![0u8]),
    };
    let properties = vec![("metadata".to_string(), metadata)];
    let res = dip721::dip721_mint(to, arg_mint, properties);
    res.unwrap().to_string().parse::<u32>().unwrap()
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
    let from = transfer_request.from;
    let to = transfer_request.to;
    let token = transfer_request.token;
    let from_pid = match from {
        User::principal(pid) => pid,
        User::address(_aid) => Principal::anonymous(),
    };
    let to_pid = match to {
        User::principal(pid) => pid,
        User::address(_aid) => Principal::anonymous(),
 
    };
    let token_obj = token_identifier::decode_token_id(&token).unwrap();
    let token_index = Nat::from(token_obj.index.get_value());
    TransferResponse::ok(dip721::dip721_transfer_from(from_pid, to_pid, token_index).unwrap())
}

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