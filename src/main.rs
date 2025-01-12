use std::time::Duration;

use futures_util::StreamExt;
use spiffe::wrapper::SpiffeWorkloadClient;
use tokio::time::sleep;

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let c = SpiffeWorkloadClient::new("/tmp/spire-agent/public/api.sock")
                .await
                .unwrap();

            loop {
                match c.fetch_x509_svid().await {
                    Ok(mut x) => match x.next().await {
                        Some(x) => x.into_iter().for_each(|x| println!("{}", x.spiffe_id())),
                        None => println!("eof"),
                    },
                    Err(e) => println!("{e:?}"),
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
}
