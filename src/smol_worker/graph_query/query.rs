#![allow(clippy::all, warnings)]
pub struct LargestSales;
pub mod largest_sales {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "LargestSales";
    pub const QUERY : & str = "query LargestSales($limit: Int!, $contract: String!) {\n  sales (first: $limit, where : {contract : $contract} orderBy: price, orderDirection: desc) {\n    timestamp\n    price\n    nft {\n      tokenId\n    }\n  }\n}\n\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type BigInt = super::BigInt;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub limit: Int,
        pub contract: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub sales: Vec<LargestSalesSales>,
    }
    #[derive(Deserialize, Debug)]
    pub struct LargestSalesSales {
        pub timestamp: BigInt,
        pub price: BigInt,
        pub nft: LargestSalesSalesNft,
    }
    #[derive(Deserialize, Debug)]
    pub struct LargestSalesSalesNft {
        #[serde(rename = "tokenId")]
        pub token_id: BigInt,
    }
}
impl graphql_client::GraphQLQuery for LargestSales {
    type Variables = largest_sales::Variables;
    type ResponseData = largest_sales::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: largest_sales::QUERY,
            operation_name: largest_sales::OPERATION_NAME,
        }
    }
}
pub struct RecentSales;
pub mod recent_sales {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RecentSales";
    pub const QUERY : & str = "query RecentSales($limit: Int!, $contract: String!) {\n  sales (first: $limit, where : {contract : $contract} orderBy: timestamp, orderDirection: desc) {\n    timestamp\n    price\n    nft {\n      tokenId\n    }\n  }\n}\n\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type BigInt = super::BigInt;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub limit: Int,
        pub contract: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub sales: Vec<RecentSalesSales>,
    }
    #[derive(Deserialize, Debug)]
    pub struct RecentSalesSales {
        pub timestamp: BigInt,
        pub price: BigInt,
        pub nft: RecentSalesSalesNft,
    }
    #[derive(Deserialize, Debug)]
    pub struct RecentSalesSalesNft {
        #[serde(rename = "tokenId")]
        pub token_id: BigInt,
    }
}
impl graphql_client::GraphQLQuery for RecentSales {
    type Variables = recent_sales::Variables;
    type ResponseData = recent_sales::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: recent_sales::QUERY,
            operation_name: recent_sales::OPERATION_NAME,
        }
    }
}
pub struct DailySales;
pub mod daily_sales {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DailySales";
    pub const QUERY : & str = "query DailySales($limit: Int!, $contract: String!, $ts: Int!) {\n    sales(first: $limit, where: { contract: $contract, timestamp_gt: $ts }, orderBy: timestamp, orderDirection: desc) {\n      price\n    }\n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type BigInt = super::BigInt;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub limit: Int,
        pub contract: String,
        pub ts: Int,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub sales: Vec<DailySalesSales>,
    }
    #[derive(Deserialize, Debug)]
    pub struct DailySalesSales {
        pub price: BigInt,
    }
}
impl graphql_client::GraphQLQuery for DailySales {
    type Variables = daily_sales::Variables;
    type ResponseData = daily_sales::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: daily_sales::QUERY,
            operation_name: daily_sales::OPERATION_NAME,
        }
    }
}

type BigInt = String;
