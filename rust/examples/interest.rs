use futures::StreamExt;
use pangea_client::{
    core::types::ChainId, provider::MoveProvider, query::Bound,
    requests::interest::GetSwapsRequest, ClientBuilder, Format, WsProvider,
};
use std::{collections::HashSet, sync::Arc};

#[tokio::main]
async fn main() {
    dotenvy::dotenv_override().ok();

    // create client
    let client = match ClientBuilder::default()
        .endpoint("movement.app.pangea.foundation")
        .build::<WsProvider>()
        .await
    {
        Ok(client) => Arc::new(client),
        Err(e) => {
            eprintln!("Client failed to initialize:\n{e}");
            return;
        }
    };

    // {
    //     let request = GetTokensRequest {
    //         chains: HashSet::from([ChainId::MOVEMENT]),
    //         from_block: Bound::Exact(0),
    //         to_block: Bound::Latest,
    //         ..Default::default()
    //     };
    //     let stream = match client
    //         .get_move_interest_v1_tokens_by_format(request, Format::JsonStream, false)
    //         .await
    //     {
    //         Ok(stream) => stream,
    //         Err(e) => {
    //             eprintln!("Request to get tokens failed\n{e}");
    //             return;
    //         }
    //     };

    //     futures::pin_mut!(stream);

    //     // async iterator over stream of data
    //     while let Some(chunk) = stream.next().await {
    //         let chunk = String::from_utf8(chunk.unwrap()).unwrap();
    //         println!("token {chunk}");
    //     }
    // }

    // {
    //     let request = GetPoolsRequest {
    //         chains: HashSet::from([ChainId::MOVEMENT]),
    //         from_block: Bound::Exact(0),
    //         to_block: Bound::Latest,
    //         ..Default::default()
    //     };
    //     let stream = match client
    //         .get_move_interest_v1_pools_by_format(request, Format::JsonStream, false)
    //         .await
    //     {
    //         Ok(stream) => stream,
    //         Err(e) => {
    //             eprintln!("Request to get pools failed\n{e}");
    //             return;
    //         }
    //     };

    //     futures::pin_mut!(stream);

    //     // async iterator over stream of data
    //     while let Some(chunk) = stream.next().await {
    //         let chunk = String::from_utf8(chunk.unwrap()).unwrap();
    //         println!("tx {chunk}");
    //     }
    // }

    // {
    //     let request = GetLiquidityRequest {
    //         chains: HashSet::from([ChainId::MOVEMENT]),
    //         from_block: Bound::FromLatest(10000),
    //         to_block: Bound::Subscribe,
    //         ..Default::default()
    //     };
    //     let stream = match client
    //         .get_move_interest_v1_liquidity_by_format(request, Format::JsonStream, false)
    //         .await
    //     {
    //         Ok(stream) => stream,
    //         Err(e) => {
    //             eprintln!("Request to get liquidity failed\n{e}");
    //             return;
    //         }
    //     };

    //     futures::pin_mut!(stream);

    //     // async iterator over stream of data
    //     while let Some(chunk) = stream.next().await {
    //         let chunk = String::from_utf8(chunk.unwrap()).unwrap();
    //         println!("tx {chunk}");
    //     }
    // }

    {
        let request = GetSwapsRequest {
            chains: HashSet::from([ChainId::MOVEMENT]),
            from_block: Bound::FromLatest(10000),
            to_block: Bound::Subscribe,
            ..Default::default()
        };
        let stream = match client
            .get_move_interest_v1_swaps_by_format(request, Format::JsonStream, false)
            .await
        {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Request to get swaps failed\n{e}");
                return;
            }
        };

        futures::pin_mut!(stream);

        // async iterator over stream of data
        while let Some(chunk) = stream.next().await {
            let chunk = String::from_utf8(chunk.unwrap()).unwrap();
            println!("tx {chunk}");
        }
    }
}
