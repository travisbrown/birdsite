#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn download_and_generate() -> Result<(), birdsite_graphql_ctid::client::Error> {
        let client = birdsite_graphql_ctid::client::Client::default();
        let site_info = client.get_site_info().await?;

        let generator = birdsite_graphql_ctid::Generator::default();

        let transaction_id = generator.compute(
            &site_info,
            "UsersByRestIds",
            "1hjT2eXW1Zcw-2xk8EbvoA",
            None,
            None,
        );

        assert_eq!(transaction_id.len(), 94);

        Ok(())
    }
}
