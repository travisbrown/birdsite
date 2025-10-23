#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn download_and_generate() -> Result<(), birdsite_graphql_ctid::client::Error> {
        let endpoint =
            birdsite_graphql_ctid::Endpoint::new("UsersByRestIds", "1hjT2eXW1Zcw-2xk8EbvoA");

        let transaction_id = birdsite_graphql_ctid::generate(&endpoint).await?;

        assert_eq!(transaction_id.value.len(), 94);

        Ok(())
    }
}
