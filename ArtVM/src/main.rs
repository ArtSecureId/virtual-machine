// Declare Virtual machine nested module
mod vm;
mod repl;

use log::info;
use crate::vm::vm::ArtVM;

fn main() {

    dotenv::dotenv().ok();
    env_logger::init();

    let vm = ArtVM::new();

    info!("Initializing ArtVM environment.")
}
