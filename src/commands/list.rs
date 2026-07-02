use crate::jmap::authenticated_client;
use crate::models::{Email, Mailbox, Output};

pub async fn list_mailboxes() -> anyhow::Result<()> {
    let mut client = authenticated_client().await?;

    let mailboxes = client.list_mailboxes().await?;
    Output::success(mailboxes).print();

    Ok(())
}

pub async fn list_emails(mailbox: &str, limit: u32) -> anyhow::Result<()> {
    let mut client = authenticated_client().await?;

    let mailbox = client.find_mailbox(mailbox).await?;
    let emails = client.list_emails(&mailbox.id, limit).await?;

    #[derive(serde::Serialize)]
    struct EmailListResponse {
        mailbox: Mailbox,
        emails: Vec<Email>,
    }

    Output::success(EmailListResponse { mailbox, emails }).print();

    Ok(())
}

pub async fn list_identities() -> anyhow::Result<()> {
    let client = authenticated_client().await?;
    let identities = client.list_identities().await?;
    Output::success(identities).print();
    Ok(())
}
