// Declare Virtual machine nested module
mod vm;
mod repl;

use log::info;
use crate::vm::vm::AsVM;

fn main() {

    dotenv::dotenv().ok();
    env_logger::init();

    let vm = AsVM::new();

    info!("Initializing asVM environment.")
}
