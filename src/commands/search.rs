use crate::jmap::authenticated_client;
use crate::models::Output;
use crate::projection::{Projection, project_many};

/// Search filter matching JMAP Email/query FilterCondition.
///
/// Address fields (`from`, `to`, `cc`, `bcc`) are `Vec<String>`: a list with
/// two or more entries becomes a JMAP OR filter on that field. Single-entry
/// lists behave identically to the old single-string form on the wire.
#[derive(Debug, Default)]
pub struct SearchFilter {
    pub text: Option<String>,
    pub from: Vec<String>,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub mailbox: Option<String>,
    pub has_attachment: bool,
    pub min_size: Option<u32>,
    pub max_size: Option<u32>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub unread: bool,
    pub flagged: bool,
}

/// Split a `--from`/`--to`/`--cc`/`--bcc` CLI value into individual addresses.
/// A single-address call passes straight through. Empty entries produced by
/// trailing commas or extra whitespace are dropped.
pub fn split_address_filter(raw: Option<String>) -> Vec<String> {
    raw.map(|s| {
        s.split(',')
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .map(String::from)
            .collect()
    })
    .unwrap_or_default()
}

pub async fn search(
    filter: SearchFilter,
    limit: u32,
    offset: u32,
    projection: Projection,
) -> anyhow::Result<()> {
    let mut client = authenticated_client().await?;

    // Resolve mailbox name to ID if specified
    let mailbox_id = if let Some(ref mailbox_name) = filter.mailbox {
        Some(client.find_mailbox(mailbox_name).await?.id)
    } else {
        None
    };

    let props = projection.jmap_properties(false);
    let props_slice = props.as_deref();

    let page = client
        .search_emails_filtered(&filter, mailbox_id.as_deref(), limit, offset, props_slice)
        .await?;
    let returned = page.emails.len() as u32;
    let truncated = page.total > offset.saturating_add(returned);
    Output::success(project_many(page.emails, &projection))
        .with_total(page.total, truncated)
        .print();

    Ok(())
}
