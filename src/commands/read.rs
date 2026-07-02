use crate::jmap::authenticated_client;
use crate::models::Output;

pub async fn mark_read(email_id: &str, read: bool) -> anyhow::Result<()> {
    let client = authenticated_client().await?;

    let email = client.get_email(email_id).await?;

    let mut keywords = email.keywords.clone();
    if read {
        keywords.insert("$seen".to_string(), true);
    } else {
        keywords.remove("$seen");
    }

    client.set_keywords(email_id, keywords).await?;

    let status = if read { "read" } else { "unread" };
    Output::<()>::success_msg(format!("Email marked as {}", status)).print();

    Ok(())
}
