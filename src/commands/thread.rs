use crate::jmap::authenticated_client;
use crate::models::Output;

pub async fn get_thread(email_id: &str) -> anyhow::Result<()> {
    let client = authenticated_client().await?;

    let emails = client.get_thread(email_id).await?;
    Output::success(emails).print();

    Ok(())
}
