#[cfg(test)]
mod test {
    use sequeda_store::{doc, Repository, StoreRepository};
    use serde::{Deserialize, Serialize};
    use std::env;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    use sequeda_store::StoreClient;

    #[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
    struct Book {
        #[serde(rename = "_id")]
        id: String,
        title: String,
        author: String,
    }

    #[tokio::test]
    async fn test_connection() {
        // example usage
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        env::set_var(sequeda_store::MONGO_PORT, "27019");
        env::set_var("RUST_LOG", "INFO");
        let store_client = StoreClient::new(String::from("test")).await.unwrap();
        let repository: StoreRepository<Book> =
            StoreRepository::get_repository(store_client, "test", "book").await;

        repository.delete_many(None).await.unwrap();
        let books = vec![
            Book {
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
                id: uuid::Uuid::new_v4().to_string(),
            },
            Book {
                title: "To Kill a Mockingbird".to_string(),
                author: "Harper Lee".to_string(),
                id: uuid::Uuid::new_v4().to_string(),
            },
        ];
        repository.insert_many(&books).await.unwrap();
        tracing::info!("from find by id");
        let book = books.first().unwrap();
        let id = &book.id;
        let result = repository.find_by_id(id).await.unwrap();
        assert!(result.is_some());
        if let Some(book) = result {
            tracing::info!(
                "found the book {}",
                serde_json::to_string_pretty(&book).unwrap()
            );
            assert_eq!(&book, books.first().unwrap());
        }
        let cursor_books = repository.find_all().await.unwrap();
        assert_eq!(books.len(), cursor_books.len());
        for book in cursor_books {
            tracing::info!("{}", serde_json::to_string_pretty(&book).unwrap());
            assert!(books.contains(&book));
        }
    }
}
