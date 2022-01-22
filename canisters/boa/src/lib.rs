#[ic_cdk_macros::update]
fn rand_bytes() -> String {
    let mut context = boa::Context::new();

    context.register_global_function(
        "rawRand",
        0,
        raw_rand
    ).unwrap();

    let rand_bytes = context.eval(r#"
        const randBytes = rawRand();

        randBytes.toString()
    "#);

    // returning a string representation of the bytes for simplicity
    rand_bytes
        .unwrap()
        .as_string()
        .unwrap()
        .to_string()
}

fn raw_rand(
    _this: &boa::JsValue,
    _aargs: &[boa::JsValue],
    _context: &mut boa::Context
) -> boa::JsResult<boa::JsValue> {
    // TODO the challenge is to get this cross-canister call to work
    // TODO I've tried many things: futures::executor::block_on, ic_cdk::block_on, messing with the ic_cdk futures implementation, etc
    // let call_result: Result<(Vec<u8>,), _> = ic_cdk::api::call::call(
    //     ic_cdk::export::Principal::management_canister(),
    //     "raw_rand",
    //     ()
    // ).await;
    
    // let rand_bytes = call_result.unwrap().0;

    let rand_bytes: Vec<u8> = vec![1, 2, 3]; // TODO comment this out and uncomment the above, if you can do that you're a hero
    
    let mut context = boa::Context::new();

    // This is a hacky way to convert a Rust Vec<u8> into a more appropriate boa JsValue
    let value = context
        .eval(
            format!(
                "Uint8Array.from({rand_bytes})",
                rand_bytes = serde_json::to_string(&rand_bytes).unwrap()
            )
        )
        .unwrap();

    Ok(value)
}

// This is simply required for boa to compile for the IC Wasm environment
fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> { Ok(()) }
getrandom::register_custom_getrandom!(custom_getrandom);