#dfx canister call lands mint "(record {to=\"7f8b2615da376df0f7651098051281835c2404439d8bb2b9b1fd8dcbcca80bd4\";slotdata=1;land_id=1;})"

#$TOKEN2=$('dfx canister call ext_based_721_backend token_identifier 1')

dfx canister  call ext_based_721_backend batch_transfer \
"(record {to=variant {\"principal\"=principal \"ztbm7-6u6jt-hgprr-7qx56-p36ca-a2qvx-y5767-4dxuc-xllss-ebtpt-sae\"};
    token= \"nelid-eakor-uwiaa-aaaaa-aaaaa-eaqca-aaaaa-q\";
     notify=true; 
     from=variant {\"principal\"=principal \"kkwoi-3jebw-6qx6z-yeah7-pgtlm-gbqdm-kkvyt-eqgbl-x3vpw-wfu2w-rqe\"};
    memo=vec {1}; 
     subaccount=opt vec {0};
      amount=1;},
      opt 3)"