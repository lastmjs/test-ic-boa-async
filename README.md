# test-ic-boa-async

This repository is a small reproducible example of a fundamental issue with boa and the Internet Computer's futures implementation.

The canister code is found in `canisters/boa/src/lib.rs`. There are comments explaining the problem.

To test your solution simply start an IC replica and then run `./test.sh` from a terminal. You may need to run `chmod +x test.sh` to enable executing `test.sh` as a script.

Running `test.sh` will simply deploy the canister locally and call the `rand_bytes` update function. By default the function should return the string `"1,2,3"`. You will need to follow the comments to enable the code that is broken.

If you can get the broken code to work, then you are a hero. It's as simple as that.