use crate::module::ledger;
use crate::module::token_identifier;
use crate::module::types::{
    CommonError, GeneralValue, InitArgs, MetaDataFungibleDetails, MetaDataNonFungibleDetails,
    NftError, TokenIdentifier, TokenMetaData, TokenMetaDataExt,
};
use cap_sdk::{insert_sync, DetailValue, IndefiniteEvent};
use ic_cdk::api::time;
use ic_cdk::export::candid::Nat;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use std::cell::RefCell;
use std::ops::Not;
use std::sync::atomic::AtomicU32;

use serde_json;
thread_local! {
    static TID: RefCell<AtomicU32> = RefCell::new(AtomicU32::new(1));
}

pub fn new_token_id() -> u32 {
    TID.with(|tid| {
        let token = tid.borrow_mut();
        let new_id = token.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        new_id
    })
}

pub fn dip721_init(args: Option<InitArgs>) {
    ledger::with_mut(|ledger| ledger.init_metadata(ic_cdk::api::caller(), args));
}

pub fn dip721_total_supply() -> Nat {
    ledger::with(|ledger| Nat::from(ledger.tokens_count()))
}

pub fn dip721_balance_of(owner: Principal) -> Result<Nat, NftError> {
    ledger::with(|ledger| {
        ledger
            .owner_token_identifier(&owner)
            .map(|token_identifier| Nat::from(token_identifier.len()))
    })
}

pub fn dip721_transfer_from(
    owner: Principal,
    to: Principal,
    token_identifier: TokenIdentifier,
) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = ic_cdk::api::caller();
        if owner.eq(&to) {
            insert_sync(IndefiniteEvent {
                caller: ic_cdk::api::caller(),
                operation: "verify owner".into(),
                details: vec![("owner".into(), DetailValue::from(owner.clone()))],
            });
            return Err(NftError::UnauthorizedOwner);
        }
        let old_owner = match ledger.owner_of(&token_identifier).ok() {
            Some(owner) => owner,
            None => return Err(NftError::OwnerNotFound),
        };
        let old_operator = match ledger.operator_of(&token_identifier).ok() {
            Some(operator) => operator,
            None => return Err(NftError::OperatorNotFound),
        };
        if old_owner.ne(&Some(owner)) {
            insert_sync(IndefiniteEvent {
                caller: ic_cdk::api::caller(),
                operation: "verify old owner".into(),
                details: vec![(
                    "old owner".into(),
                    DetailValue::from(old_owner.unwrap().clone()),
                )],
            });
            return Err(NftError::UnauthorizedOwner);
        }
        if old_operator.ne(&Some(caller)) {
            insert_sync(IndefiniteEvent {
                caller: ic_cdk::api::caller(),
                operation: "verify old operator".into(),
                details: vec![(
                    "old operator".into(),
                    DetailValue::from(old_operator.unwrap().clone()),
                )],
            });
            return Err(NftError::UnauthorizedOperator);
        }

        ledger.update_owner_cache(&token_identifier, old_owner, Some(to));
        ledger.update_operator_cache(&token_identifier, old_operator, Some(to));
        ledger.transfer(caller, &token_identifier, Some(to));

        insert_sync(IndefiniteEvent {
            caller,
            operation: "transferFrom".into(),
            details: vec![
                ("owner".into(), DetailValue::from(owner)),
                ("to".into(), DetailValue::from(to)),
                (
                    "token_identifier".into(),
                    DetailValue::from(token_identifier.to_string()),
                ),
            ],
        });

        Ok(Nat::from(ledger.inc_tx() - 1))
    })
}

pub fn dip721_mint(
    to: Principal,
    token_identifier: TokenIdentifier,
    properties: Vec<(String, GeneralValue)>,
) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = ic_cdk::api::caller();
        if properties.is_empty() {
            insert_sync(IndefiniteEvent {
                caller: ic_cdk::api::caller(),
                operation: "verify properites".into(),
                details: vec![(
                    "properties has no metadata".into(),
                    DetailValue::from(token_identifier.clone()),
                )],
            });
        }
        if !ledger.is_token_existed(&token_identifier).not() {
            insert_sync(IndefiniteEvent {
                caller: ic_cdk::api::caller(),
                operation: "verify token exist".into(),
                details: vec![(
                    "existed token identifier".into(),
                    DetailValue::from(token_identifier.clone()),
                )],
            });
            return Err(NftError::ExistedNFT);
        }
        ledger.add_token_metadata(
            token_identifier.clone(),
            TokenMetaData {
                token_identifier: token_identifier.clone(),
                owner: Some(to),
                operator: Some(to),
                properties,
                is_burned: false,
                minted_at: time(),
                minted_by: caller,
                transferred_at: None,
                transferred_by: None,
                approved_at: None,
                approved_by: None,
                burned_at: None,
                burned_by: None,
            },
        );
        ledger.update_owner_cache(&token_identifier, None, Some(to));
        ledger.update_operator_cache(&token_identifier, None, Some(to));
        insert_sync(IndefiniteEvent {
            caller,
            operation: "mint".into(),
            details: vec![
                ("to".into(), DetailValue::from(to)),
                (
                    "token_identifier".into(),
                    DetailValue::from(token_identifier.to_string()),
                ),
            ],
        });

        Ok(Nat::from(ledger.inc_tx() - Nat::from(1)))
    })
}

pub fn dip721_burn(token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = ic_cdk::api::caller();
        let old_owner = match ledger.owner_of(&token_identifier).ok() {
            Some(owner) => owner,
            None => return Err(NftError::OwnerNotFound),
        };
        if old_owner.ne(&Some(caller)) {
            insert_sync(IndefiniteEvent {
                caller: ic_cdk::api::caller(),
                operation: "verify old owner".into(),
                details: vec![(
                    "unauthozied owner".into(),
                    DetailValue::from(caller.clone()),
                )],
            });
            return Err(NftError::UnauthorizedOwner);
        }
        let old_operator = match ledger.operator_of(&token_identifier).ok() {
            Some(operator) => operator,
            None => return Err(NftError::OperatorNotFound),
        };
        ledger.update_owner_cache(&token_identifier, old_owner, None);
        ledger.update_operator_cache(&token_identifier, old_operator, None);
        ledger.burn(caller, &token_identifier);

        insert_sync(IndefiniteEvent {
            caller,
            operation: "burn".into(),
            details: vec![(
                "token_identifier".into(),
                DetailValue::from(token_identifier.to_string()),
            )],
        });

        Ok(Nat::from(ledger.inc_tx() - 1))
    })
}

pub fn dip721_approve(
    operator: Principal,
    token_identifier: TokenIdentifier,
) -> Result<Nat, NftError> {
    ledger::with_mut(|ledger| {
        let caller = ic_cdk::api::caller();
        if operator.eq(&caller) {
            insert_sync(IndefiniteEvent {
                caller: ic_cdk::api::caller(),
                operation: "verify caller".into(),
                details: vec![("operator".into(), DetailValue::from(operator.to_string()))],
            });
            return Err(NftError::SelfApprove);
        };
        let owner = match ledger.owner_of(&token_identifier).ok() {
            Some(owner) => owner,
            None => return Err(NftError::OwnerNotFound),
        };
        if owner.ne(&Some(caller)) {
            insert_sync(IndefiniteEvent {
                caller: ic_cdk::api::caller(),
                operation: "verify owner".into(),
                details: vec![(
                    "owner".into(),
                    DetailValue::from(owner.unwrap().to_string()),
                )],
            });
            return Err(NftError::UnauthorizedOwner);
        }
        ledger.update_operator_cache(
            &token_identifier,
            ledger.operator_of(&token_identifier)?,
            Some(operator),
        );
        ledger.approve(caller, &token_identifier, Some(operator));

        insert_sync(IndefiniteEvent {
            caller,
            operation: "approve".into(),
            details: vec![
                ("operator".into(), DetailValue::from(operator)),
                (
                    "token_identifier".into(),
                    DetailValue::from(token_identifier.to_string()),
                ),
            ],
        });

        Ok(Nat::from(ledger.inc_tx() - 1))
    })
}

pub fn dip721_token_metadata(token_identifier: TokenIdentifier) -> Result<TokenMetaData, NftError> {
    ledger::with(|ledger| ledger.token_metadata(&token_identifier).cloned())
}

//pub fn dip721_owner_of(token_identifier: )
