use anyhow::Error;
use anyhow::Result as AnyhowResult;

mod smol_worker;
mod base_worker;
mod utils;

use crate::base_worker::BaseWorker;
use crate::smol_worker::SmolWorker;

#[tokio::main]
async fn main() -> AnyhowResult<(), Error> {
    let worker = smol_worker::SmolWorker::new("configs/smol_apa_config.json");
    let worker2: SmolWorker = smol_worker::SmolWorker::new("configs/smol_land_config.json");
    worker.execute().unwrap();
    worker2.execute().unwrap();
    println!("Hello, world!");
    Ok(())
}

