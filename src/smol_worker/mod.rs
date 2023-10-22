mod config;
mod graph_query;

use anyhow::Error;

use anyhow::anyhow;
use crate::utils::perform_generic_query;


use graph_query::query::daily_sales;
use crate::base_worker::BaseWorker;
use config::Config;
use graph_query::query::DailySales;
use crate::smol_worker::graph_query::query::daily_sales::DailySalesSales;

pub struct SmolWorker {
    config: Config,
}

impl BaseWorker for SmolWorker {
    fn new(config_json_path: &str) -> Self {
        let config = SmolWorker::read_json_into_string(config_json_path);
        SmolWorker { config }
    }

    fn execute(&self) -> Result<(), &'static str> {
        println!("{:?}", self.config);
        Ok(())
    }
}

impl SmolWorker {
    async fn query_daily_sales(&self) -> Result<Vec<DailySalesSales>, Error> {
        let contract_address = &self.config.collection_address; // if you dont use ref, it says cannot move, this indacates any sorta assigment is a move
        let variables = daily_sales::Variables { limit: 5, contract: contract_address.to_owned(), ts: 1665985624 };
        let sales_list = perform_generic_query::<DailySales>(variables)
            .await?
            .data
            .ok_or(anyhow!("Query failed"))?
            .sales;
        println!("{:?}", sales_list);
        Ok(sales_list)
    }
}

