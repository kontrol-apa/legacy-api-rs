use anyhow::Error;
use std::fmt::Debug;
use graphql_client::{GraphQLQuery, Response};

pub async fn perform_generic_query<Q: GraphQLQuery>(
    variables: Q::Variables,
) -> Result<Response<Q::ResponseData>, Error>
    where
        <Q as GraphQLQuery>::ResponseData: Debug { // Add this trait bound

    let request_body = Q::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.thegraph.com/subgraphs/name/traderjoe-xyz/marketplace")
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response<Q::ResponseData> = res.json().await?;

    println!("{:?}", response_body);
    Ok(response_body)
}