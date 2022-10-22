#[cfg(test)]
mod test {

    use sequeda_common::exchange::Exchange;
    use sequeda_message_client::MessageClient;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    #[tokio::test]
    async fn test_client() {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        let mut fut = vec![];
        for i in 0..100 {
            fut.push(tokio::spawn(async move {
                tracing::info!("waiting..");
                let mut client = MessageClient::new(&format!("person{i}")).await.unwrap();
                tracing::info!("stop waiting");
                client
                    .send(Exchange::new("Hello World", "Animal"))
                    .await
                    .unwrap();
                drop(client);
            }));
        }
        futures_util::future::join_all(fut).await;

        let mut client = MessageClient::new("person_sub").await.unwrap();
        client.subscribe("Animal").await.unwrap();
        let mut count = 0;
        while let Some(Ok(msg)) = client.recv().await {
            tracing::info!("{msg:?}");
            count += 1;
        }

        assert_eq!(100, count);
    }
}
