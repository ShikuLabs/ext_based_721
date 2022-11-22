# ext_based_721

Welcome to your new ext_based_721 project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with ext_based_721, see the following documentation available online:

- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd ext_based_721/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.
## interface 
###   batch_transfer : (TransferRequest, opt nat32) -> (TransferResponse);;
```
input params: 
"(record {to=variant {\"principal\"=principal \"nuppp-6pngd-jxnv2-ko3ah-ippt6-pqex5-avwxl-lctvt-fhwn6-esmnr-uqe\"}; //dest user pid
    token= \"2h4pg-aikor-uwiaa-aaaaa-byakr-iaqca-aaaab-a\";                                                         //encoded token id, can get from 'token_identifier interface'
     notify=true; 
     from=variant {\"principal\"=principal \"gze77-i3egd-wbuoy-zn27p-wv5ze-casv2-w4miv-skbzu-eil5w-uacl7-xae\"};    //src user pid
    memo=vec {1}; 
     subaccount=opt vec {0};
      amount=1;},
      opt 20)"                                                                                                      //nums for transfer
```
###   token_identifier : (nat) -> (text) query;
Get the encoded token corresponding to the token identifier.  
###  transfer : (TransferRequest) -> (TransferResponse);
```
  add game detail
input params: 
type TransferRequest = record {
  to : User;
  token : text;
  notify : bool;
  from : User;
  memo : vec nat8;
  subaccount : opt vec nat8;
  amount : nat;
};

demo:
"(record {to=variant {\"principal\"=principal \"nuppp-6pngd-jxnv2-ko3ah-ippt6-pqex5-avwxl-lctvt-fhwn6-esmnr-uqe\"}; //dest user pid
    token= \"2h4pg-aikor-uwiaa-aaaaa-byakr-iaqca-aaaab-a\";                                                         //encoded token id, can get from 'token_identifier interface'
     notify=true; 
     from=variant {\"principal\"=principal \"gze77-i3egd-wbuoy-zn27p-wv5ze-casv2-w4miv-skbzu-eil5w-uacl7-xae\"};    //src user pid
    memo=vec {1}; 
     subaccount=opt vec {0};
      amount=1;})" 
```
###  set_developer : (text, TeamInfo) -> (GameInfo);
```
 set game developer information
input params: text =>game id. to find out the game id which you want to set developer info
Teaminfo => the developer information 
```
###  set_name : (text, text) -> (GameInfo);
```
 set game name
 input params: text =>game id. to find out the game id which you want to set the game name
 text => the game name
```
###  set_publisher : (text, TeamInfo) -> (GameInfo); 
```
set game publisher informatioin
input params: text =>game id. to find out the game id which you want to set publish info
Teaminfo => the publish information 
```
###  set_url : (text, text) -> (GameInfo);
```
 set game url
input params:text =>game id. to find out the game id which you want to set the game url
text => the game url 
```