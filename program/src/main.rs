#![no_main]

extern crate alloc;

use contract::Counter;
use sdk::guest::execute;
use sdk::guest::GuestEnv;
use sdk::guest::SP1Env;

sp1_zkvm::entrypoint!(main);

fn main() {
    //
    // Usually you don't need to update this file.
    // Except to specify the name of your contract type (here = Counter)
    //

    let env = SP1Env {};
    let input = env.read();
    let (_, output) = execute::<Counter>(&input);
    env.commit(&output);
}
