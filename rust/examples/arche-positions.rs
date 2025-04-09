use futures::StreamExt;
use pangea_client::{
    core::types::ChainId, provider::MoveProvider, query::Bound,
    requests::arche::GetPositionsRequest, ClientBuilder, Format, WsProvider,
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

    {
        let request = GetPositionsRequest {
            chains: HashSet::from([ChainId::MOVEMENT]),
            from_block: Bound::Exact(0),
            to_block: Bound::Subscribe,
            ..Default::default()
        };
        let stream = match client
            .get_move_arche_positions_by_format(request, Format::JsonStream, false)
            .await
        {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Request to get positions failed\n{e}");
                return;
            }
        };

        futures::pin_mut!(stream);

        // async iterator over stream of data
        while let Some(chunk) = stream.next().await {
            let chunk = String::from_utf8(chunk.unwrap()).unwrap();
            println!("token {chunk}");
        }
    }
}
