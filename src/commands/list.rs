use crate::jmap::authenticated_client;
use crate::models::{Mailbox, Output};
use crate::projection::{Projection, project_many};

pub async fn list_mailboxes() -> anyhow::Result<()> {
    let mut client = authenticated_client().await?;

    let mailboxes = client.list_mailboxes().await?;
    Output::success(mailboxes).print();

    Ok(())
}

pub async fn list_emails(mailbox: &str, limit: u32, projection: Projection) -> anyhow::Result<()> {
    let mut client = authenticated_client().await?;

    let mailbox = client.find_mailbox(mailbox).await?;
    let props = projection.jmap_properties(false);
    let props_slice = props.as_deref();
    let page = client.list_emails(&mailbox.id, limit, props_slice).await?;

    #[derive(serde::Serialize)]
    struct EmailListResponse {
        mailbox: Mailbox,
        emails: Vec<serde_json::Value>,
    }

    let returned = page.emails.len() as u32;
    let truncated = page.total > returned;

    Output::success(EmailListResponse {
        mailbox,
        emails: project_many(page.emails, &projection),
    })
    .with_total(page.total, truncated)
    .print();

    Ok(())
}

pub async fn list_identities() -> anyhow::Result<()> {
    let client = authenticated_client().await?;
    let identities = client.list_identities().await?;
    Output::success(identities).print();
    Ok(())
}
