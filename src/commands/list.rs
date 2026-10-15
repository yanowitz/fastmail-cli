use crate::jmap::authenticated_client;
use crate::models::{Mailbox, Output};
use crate::projection::{Projection, project_many};

pub async fn list_mailboxes() -> anyhow::Result<()> {
    let mut client = authenticated_client().await?;

    let mailboxes = client.list_mailboxes().await?;
    Output::success(mailboxes).print();

    Ok(())
}

pub async fn list_emails(
    mailbox: &str,
    limit: u32,
    offset: u32,
    projection: Projection,
) -> anyhow::Result<()> {
    let mut client = authenticated_client().await?;

    let mailbox = client.find_mailbox(mailbox).await?;
    let props = projection.jmap_properties(false);
    let props_slice = props.as_deref();
    let page = client
        .list_emails(&mailbox.id, limit, offset, props_slice)
        .await?;

    #[derive(serde::Serialize)]
    struct EmailListResponse {
        mailbox: Mailbox,
        emails: Vec<serde_json::Value>,
    }

    let returned = page.emails.len() as u64;

    let mut output = Output::success(EmailListResponse {
        mailbox,
        emails: project_many(page.emails, &projection),
    });
    if let Some(total) = page.total {
        output = output.with_total(total, total > u64::from(offset) + returned);
    }
    output.print();

    Ok(())
}

pub async fn list_identities() -> anyhow::Result<()> {
    let client = authenticated_client().await?;
    let identities = client.list_identities().await?;
    Output::success(identities).print();
    Ok(())
}
