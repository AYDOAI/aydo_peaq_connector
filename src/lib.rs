pub async fn run(url: &str) -> anyhow::Result<()> {
    hub::run(url).await?;
    Ok(())
}
