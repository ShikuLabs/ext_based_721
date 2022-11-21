use crate::token_identifier;
use ic_cdk::export::candid::{CandidType, Deserialize, Int, Nat};
use ic_cdk::export::Principal;
use serde::Serialize;
use std::collections::HashSet;

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub name: Option<String>,
    pub logo: Option<String>,
    pub symbol: Option<String>,
    pub custodians: Option<HashSet<Principal>>,
    pub cap: Option<Principal>,
}

#[derive(CandidType, Default, Deserialize)]
pub struct MetaData {
    pub name: Option<String>,
    pub logo: Option<String>,
    pub symbol: Option<String>,
    pub custodians: HashSet<Principal>,
    pub created_at: u64,
    pub upgraded_at: u64,
}

#[derive(CandidType)]
pub struct Status {
    pub total_transactions: Nat,
    pub total_supply: Nat,
    pub cycles: Nat,
    pub total_unique_holders: Nat,
}

pub type TokenIdentifier = Nat;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub enum GeneralValue {
    BoolContent(bool),
    TextContent(String),
    BlobContent(Vec<u8>),
    Principal(Principal),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    NatContent(Nat),
    Int8Content(i8),
    Int16Content(i16),
    Int32Content(i32),
    Int64Content(i64),
    IntContent(Int),
    FloatContent(f64),
    NestedContent(Vec<(String, GeneralValue)>),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TokenMetaData {
    pub token_identifier: TokenIdentifier,
    pub owner: Option<Principal>,
    pub operator: Option<Principal>,
    pub is_burned: bool,
    pub properties: Vec<(String, GeneralValue)>,
    pub minted_at: u64,
    pub minted_by: Principal,
    pub transferred_at: Option<u64>,
    pub transferred_by: Option<Principal>,
    pub approved_at: Option<u64>,
    pub approved_by: Option<Principal>,
    pub burned_at: Option<u64>,
    pub burned_by: Option<Principal>,
}

#[derive(Debug, CandidType)]
pub enum NftError {
    UnauthorizedOwner,
    UnauthorizedOperator,
    OwnerNotFound,
    OperatorNotFound,
    TokenNotFound,
    ExistedNFT,
    SelfApprove,
    SelfTransfer,
}

/////////////// YUMI TYPES ////////////

pub type Time = Int;
pub type TokenIndex = u32;
#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct SubAccount(pub Vec<u8>);

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum User {
    #[allow(non_camel_case_types)]
    address(AccountIdentifier),
    #[allow(non_camel_case_types)]
    principal(Principal),
}

impl User {
    pub fn aid(user: User) -> AccountIdentifier {
        match user {
            Self::address(aid) => aid.clone(),
            Self::principal(pid) => pid2aid(&pid),
        }
    }
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct AllowanceRequest {
    pub owner: User,
    pub spender: Principal,
    pub token: TokenIdentifier__1,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct MintRequest {
    pub to: User,
    pub metadata: Option<Vec<u8>>,
    pub class: String,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct ApproveRequest {
    pub allowance: Balance,
    pub spender: Principal,
    pub subaccount: Option<SubAccount>,
    pub token: token_identifier::TokenIdentifier,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result__1_1 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(AccountIdentifier__1),
}

#[allow(non_camel_case_types)]
pub type AccountIdentifier__1 = String;
pub type AccountIdentifier = String;
#[allow(non_camel_case_types)]
pub type TokenIdentifier__1 = String;
#[allow(non_camel_case_types)]
pub type Balance = Nat;

#[derive(Debug, CandidType, Clone, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CommonError__1 {
    InvalidToken(String),
    Other(String),
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum CommonError {
    InvalidToken(String),
    Other(String),
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct MetaDataFungibleDetails {
    decimals: u8,
    metadata: Option<Vec<u8>>,
    name: String,
    symbol: String,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct MetaDataNonFungibleDetails {
    pub metadata: Option<Vec<u8>>,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum TokenMetaDataExt {
    #[allow(non_camel_case_types)]
    fungible(MetaDataFungibleDetails),
    #[allow(non_camel_case_types)]
    nonfungible(MetaDataNonFungibleDetails),
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct Listing {
    locked: Option<Time>,
    price: u64,
    seller: Principal,
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub struct Registry(TokenIndex, AccountIdentifier__1);

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum TransferResponse {
    #[allow(non_camel_case_types)]
    err(TransferResponseDetails),
    #[allow(non_camel_case_types)]
    ok(Balance),
}

#[derive(Debug, CandidType, Clone, Deserialize)]
pub enum TransferResponseDetails {
    CannotNotify(AccountIdentifier),
    InsufficientBalance,
    InvalidToken(String),
    Other(String),
    Rejected,
    Unauthorized(AccountIdentifier),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct TransferRequest {
    pub amount: Balance,
    pub from: User,
    pub memo: Memo,
    pub notify: bool,
    pub subaccount: Option<SubAccount>,
    pub to: User,
    pub token: token_identifier::TokenIdentifier,
}

pub type Memo = Vec<u8>;

#[derive(Debug, Clone, CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result__1_2 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(Balance__1),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result__1 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(TokenMetaDataExt),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_2 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(Balance__1),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Result_1 {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(Vec<TokenIndex>),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct ResultDetail(pub TokenIndex, pub Option<Listing>, pub Option<Vec<u8>>);

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum NFTResult {
    #[allow(non_camel_case_types)]
    err(CommonError),
    #[allow(non_camel_case_types)]
    ok(Vec<ResultDetail>),
}
#[allow(non_camel_case_types)]
pub type Balance__1 = Nat;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum BalanceResponse {
    #[allow(non_camel_case_types)]
    err(CommonError__1),
    #[allow(non_camel_case_types)]
    ok(Balance),
}
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct BalanceRequest {
    token: u64,
    user: User,
}

pub fn pid2aid(pid: &Principal) -> String {
    let sub_acc = ic_ledger_types::Subaccount([0u8; 32]);
    let account_id = ic_ledger_types::AccountIdentifier::new(pid, &sub_acc);
    //  match AccountIdentifier_shiku::from_hex(&account_id.to_string()) {
    //      Ok(shiku) => shiku,
    //      Err(_) => AccountIdentifier_shiku::default(),
    //  }
    account_id.to_string()
}
