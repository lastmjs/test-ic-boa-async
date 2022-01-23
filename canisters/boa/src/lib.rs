#[ic_cdk_macros::update]
async fn rand_bytes() -> String {
    let mut context = boa::Context::new();

    context.register_global_function(
        "rawRand",
        0,
        raw_rand
    ).unwrap();

    let args = context.eval(r#"
        // Pretend that raw_rand takes arguments, for the sake of demonstration.
        rawRand("hello", true, 42, function (randBytes) {
          // Do something with randBytes. Since we are converting it to a string
          // for convenience in this demo, add a prefix message to demonstrate
          // that we are legitimately going through this JS callback function.
          return "Random bytes via JS callback: " + randBytes;
        });
    "#).unwrap();

    let args = args.as_object().unwrap();

    // Convert JavaScript values to Rust values
    let mut first_arg_context = boa::Context::new();
    let first_arg_js_value = args.get("0", &mut first_arg_context).unwrap();
    let first_arg_js_string = first_arg_js_value.as_string().unwrap();
    let first_arg_str = first_arg_js_string.as_str();

    // Do the same for the remaining args...

    let mut callback_context = boa::Context::new();
    let callback_js_value = args.get("3", &mut callback_context).unwrap();
    let callback_js_object = callback_js_value.as_object().unwrap();

    let call_result: Result<(Vec<u8>,), _> = ic_cdk::api::call::call(
        ic_cdk::export::Principal::management_canister(),
        "raw_rand",
        () // Pass args here if the function takes arguments?
    ).await;

    let rand_bytes = call_result.unwrap().0;

    // This is a hacky way to convert a Rust Vec<u8> into a more appropriate boa JsValue
    let value = context
        .eval(
            format!(
                "Uint8Array.from({rand_bytes}).toString()",
                rand_bytes = serde_json::to_string(&rand_bytes).unwrap()
            )
        )
        .unwrap();

    let from_callback = callback_js_object.call(&callback_js_value, &vec![value], &mut context).unwrap();
    let from_callback = from_callback.as_string().unwrap();
    from_callback.as_str().to_string()
}

fn raw_rand(
    _this: &boa::JsValue,
    aargs: &[boa::JsValue],
    _context: &mut boa::Context
) -> boa::JsResult<boa::JsValue> {
    // raw_rand doesn't actually take any arguments, but for functions that do
    // the goal is to simply return aargs.

    // There isn't an Array variant of boa::JsValue but
    // https://github.com/boa-dev/boa/pull/1746 might help.

    // For now we return an object where the key is the index of the argument.
    let mut context = boa::Context::new();
    let mut args = boa::object::ObjectInitializer::new(&mut context);

    aargs.iter().enumerate().for_each(|(i, x)| {
        args.property(format!("{}", i), x, boa::property::Attribute::PERMANENT);
    });

    let args = args.build();
    Ok(boa::JsValue::Object(args))
}

// This is simply required for boa to compile for the IC Wasm environment
fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> { Ok(()) }
getrandom::register_custom_getrandom!(custom_getrandom);
