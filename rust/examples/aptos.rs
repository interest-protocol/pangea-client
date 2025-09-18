use futures::StreamExt;
use pangea_client::{
    core::types::ChainId,
    provider::MoveProvider,
    query::Bound,
    requests::movement::{GetMoveLogsRequest, GetMoveTxsRequest},
    ClientBuilder, Format, WsProvider,
};
use std::{collections::HashSet, sync::Arc};

#[tokio::main]
async fn main() {
    dotenvy::dotenv_override().ok();

    // create client
    let client = match ClientBuilder::default()
        .endpoint("aptos.app.pangea.foundation")
        .build::<WsProvider>()
        .await
    {
        Ok(client) => Arc::new(client),
        Err(e) => {
            eprintln!("Client failed to initialize:\n{e}");
            return;
        }
    };

    {
        let request = GetMoveLogsRequest {
            chains: HashSet::from([ChainId::APTOS]),
            from_block: Bound::FromLatest(1000),
            to_block: Bound::Subscribe,
            module__in: HashSet::from(["transaction_fee".to_string()]),
            ..Default::default()
        };
        let stream = match client
            .get_move_logs_by_format(request, Format::JsonStream, false)
            .await
        {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Request to get logs failed\n{e}");
                return;
            }
        };

        futures::pin_mut!(stream);

        // async iterator over stream of data
        while let Some(chunk) = stream.next().await {
            let chunk = String::from_utf8(chunk.unwrap()).unwrap();
            println!("logs {chunk}");
        }
    }

    {
        let request = GetMoveLogsRequest {
            chains: HashSet::from([ChainId::APTOS]),
            from_block: Bound::FromLatest(1000),
            to_block: Bound::Latest,
            module__in: HashSet::from(["transaction_fee".to_string()]),
            ..Default::default()
        };
        let stream = match client
            .get_move_logs_decoded_by_format(request, Format::JsonStream, false)
            .await
        {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Request to get logs failed\n{e}");
                return;
            }
        };

        futures::pin_mut!(stream);

        // async iterator over stream of data
        while let Some(chunk) = stream.next().await {
            let chunk = String::from_utf8(chunk.unwrap()).unwrap();
            println!("logs dec {chunk}");
        }
    }

    {
        let request = GetMoveTxsRequest {
            chains: HashSet::from([ChainId::APTOS]),
            from_block: Bound::FromLatest(10),
            to_block: Bound::Latest,
            ..Default::default()
        };
        let stream = match client
            .get_move_txs_by_format(request, Format::JsonStream, false)
            .await
        {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Request to get txs failed\n{e}");
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
