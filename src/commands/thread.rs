use crate::jmap::authenticated_client;
use crate::models::Output;
use crate::projection::{Projection, project_many};

pub async fn get_thread(email_id: &str, projection: Projection) -> anyhow::Result<()> {
    let client = authenticated_client().await?;

    let props = projection.jmap_properties(true);
    let props_slice = props.as_deref();
    let fetch_bodies = projection.wants_body_values();

    let emails = client
        .get_thread(email_id, props_slice, fetch_bodies)
        .await?;
    Output::success(project_many(emails, &projection)).print();

    Ok(())
}
