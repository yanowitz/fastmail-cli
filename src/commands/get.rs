use crate::jmap::authenticated_client;
use crate::models::Output;

pub async fn get_email(email_id: &str) -> anyhow::Result<()> {
    let client = authenticated_client().await?;

    let email = client.get_email(email_id).await?;
    Output::success(email).print();

    Ok(())
}
