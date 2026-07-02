//! Output projection: `Full`, `Compact` (curated agent-friendly preset), or
//! explicit `Fields(...)` JMAP passthrough. Keeps the wire-level response shape
//! agent-friendly without changing the default behaviour.
//!
//! The projection also drives the JMAP `properties` argument, so slimmer output
//! means fewer bytes fetched from the server — not just fewer bytes rendered.

use crate::models::{CompactEmail, Email};
use serde_json::{Map, Value};

/// Every JMAP Email property the CLI accepts in `--fields`.
///
/// The spec defines a few more (e.g. `sender`, `headers`, `bodyStructure`) that
/// we don't currently surface. Extend here when new fields are needed.
pub const EMAIL_PROPERTIES: &[&str] = &[
    "id",
    "blobId",
    "threadId",
    "mailboxIds",
    "keywords",
    "size",
    "receivedAt",
    "sentAt",
    "messageId",
    "inReplyTo",
    "references",
    "from",
    "to",
    "cc",
    "bcc",
    "replyTo",
    "subject",
    "preview",
    "hasAttachment",
    "textBody",
    "htmlBody",
    "attachments",
    "bodyValues",
];

/// Fields fetched for `--compact` on `search` / `list emails` (no bodies).
const COMPACT_SUMMARY: &[&str] = &[
    "id",
    "threadId",
    "mailboxIds",
    "keywords",
    "size",
    "receivedAt",
    "from",
    "to",
    "subject",
    "preview",
    "hasAttachment",
];

/// Fields fetched for `--compact` on `get` / `thread` (summary + bodies).
///
/// `replyTo` is deliberately omitted: [`CompactEmail`] doesn't surface it, so
/// fetching it would just waste JMAP bandwidth.
const COMPACT_FULL: &[&str] = &[
    "id",
    "threadId",
    "mailboxIds",
    "keywords",
    "size",
    "receivedAt",
    "from",
    "to",
    "cc",
    "subject",
    "preview",
    "hasAttachment",
    "textBody",
    "htmlBody",
    "attachments",
    "bodyValues",
];

#[derive(Debug, Clone)]
pub enum Projection {
    /// Preserve current behaviour: every property the command used to fetch.
    Full,
    /// Curated agent-friendly shape (see [`CompactEmail`]).
    Compact,
    /// JMAP passthrough; caller-validated list of property names.
    Fields(Vec<String>),
}

impl Projection {
    /// Parse and validate the `--fields` CSV. Rejects unknown names, empty
    /// input, and synthetic names (e.g. `unread`, `flagged`). `id` is forced
    /// in because every downstream action needs it.
    pub fn from_fields_csv(csv: &str) -> Result<Self, String> {
        let mut fields: Vec<String> = csv
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if fields.is_empty() {
            return Err("--fields requires at least one field name".into());
        }
        for f in &fields {
            if !EMAIL_PROPERTIES.contains(&f.as_str()) {
                return Err(format!(
                    "unknown field '{}' (use --compact for derived fields like 'unread'/'flagged'). Valid: {}",
                    f,
                    EMAIL_PROPERTIES.join(", ")
                ));
            }
        }
        if !fields.iter().any(|f| f == "id") {
            fields.insert(0, "id".to_string());
        }
        Ok(Self::Fields(fields))
    }

    /// Properties to request from JMAP, or `None` to let the caller use its
    /// own default list. `include_bodies` picks the compact variant.
    pub fn jmap_properties(&self, include_bodies: bool) -> Option<Vec<&str>> {
        match self {
            Self::Full => None,
            Self::Compact => Some(
                if include_bodies {
                    COMPACT_FULL
                } else {
                    COMPACT_SUMMARY
                }
                .to_vec(),
            ),
            Self::Fields(fs) => Some(fs.iter().map(String::as_str).collect()),
        }
    }

    /// Whether JMAP should be told to populate `bodyValues`. For `Fields`,
    /// only if the user asked for it explicitly.
    pub fn wants_body_values(&self) -> bool {
        match self {
            Self::Full | Self::Compact => true,
            Self::Fields(fs) => fs.iter().any(|s| s == "bodyValues"),
        }
    }
}

/// Apply a projection to a single email, producing the JSON value that will be
/// serialized to the user.
pub fn project_email(email: Email, proj: &Projection) -> Value {
    match proj {
        Projection::Full => serde_json::to_value(email).unwrap_or(Value::Null),
        Projection::Compact => {
            serde_json::to_value(CompactEmail::from(email)).unwrap_or(Value::Null)
        }
        Projection::Fields(fs) => {
            let v = serde_json::to_value(email).unwrap_or(Value::Null);
            let Value::Object(map) = v else {
                return v;
            };
            let mut out = Map::with_capacity(fs.len());
            for f in fs {
                if let Some(val) = map.get(f) {
                    out.insert(f.clone(), val.clone());
                }
            }
            Value::Object(out)
        }
    }
}

/// Apply a projection to a list of emails.
pub fn project_many(emails: Vec<Email>, proj: &Projection) -> Vec<Value> {
    emails.into_iter().map(|e| project_email(e, proj)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Email, EmailAddress, EmailBodyPart, EmailBodyValue};
    use std::collections::HashMap;

    fn sample_email() -> Email {
        let mut keywords = HashMap::new();
        keywords.insert("$seen".to_string(), true);
        keywords.insert("$flagged".to_string(), true);
        let mut mailbox_ids = HashMap::new();
        mailbox_ids.insert("P5F".to_string(), true);
        Email {
            id: "id1".to_string(),
            blob_id: Some("blob1".to_string()),
            thread_id: Some("t1".to_string()),
            mailbox_ids,
            keywords,
            size: 1024,
            received_at: Some("2024-01-01T00:00:00Z".to_string()),
            message_id: None,
            in_reply_to: None,
            references: None,
            from: Some(vec![EmailAddress {
                name: Some("A".into()),
                email: "a@x".into(),
            }]),
            to: None,
            cc: None,
            bcc: None,
            reply_to: None,
            subject: Some("Hi".to_string()),
            sent_at: None,
            preview: Some("preview text".to_string()),
            has_attachment: false,
            text_body: None,
            html_body: None,
            attachments: None,
            body_values: None,
        }
    }

    #[test]
    fn compact_derives_unread_flagged_and_drops_internals() {
        let email = sample_email();
        let v = project_email(email, &Projection::Compact);
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("unread").and_then(Value::as_bool), Some(false));
        assert_eq!(obj.get("flagged").and_then(Value::as_bool), Some(true));
        assert!(obj.get("mailboxIds").is_none());
        assert!(obj.get("keywords").is_none());
        assert!(obj.get("blobId").is_none());
    }

    #[test]
    fn compact_flattens_text_body() {
        let mut email = sample_email();
        let mut bv = HashMap::new();
        bv.insert(
            "1".to_string(),
            EmailBodyValue {
                value: "hello world".into(),
                is_encoding_problem: false,
                is_truncated: false,
            },
        );
        email.body_values = Some(bv);
        email.text_body = Some(vec![EmailBodyPart {
            part_id: Some("1".into()),
            blob_id: None,
            size: 11,
            name: None,
            content_type: Some("text/plain".into()),
            charset: None,
            disposition: None,
            cid: None,
        }]);

        let v = project_email(email, &Projection::Compact);
        assert_eq!(
            v.get("textBody").and_then(Value::as_str),
            Some("hello world")
        );
    }

    #[test]
    fn compact_strips_html_when_textbody_part_is_text_html() {
        // JMAP reports the HTML part in `textBody` when no plain-text part
        // exists. textBody[0].type is "text/html", not "text/plain". We must
        // strip the HTML before returning it as the compact `textBody` field.
        let mut email = sample_email();
        let mut bv = HashMap::new();
        bv.insert(
            "1".to_string(),
            EmailBodyValue {
                value: "<html><body><p>Hello <b>world</b></p></body></html>".into(),
                is_encoding_problem: false,
                is_truncated: false,
            },
        );
        email.body_values = Some(bv);
        email.text_body = Some(vec![EmailBodyPart {
            part_id: Some("1".into()),
            blob_id: None,
            size: 50,
            name: None,
            content_type: Some("text/html".into()),
            charset: None,
            disposition: None,
            cid: None,
        }]);

        let v = project_email(email, &Projection::Compact);
        let body = v.get("textBody").and_then(Value::as_str);
        assert_eq!(body, Some("Hello world"));
    }

    #[test]
    fn compact_falls_back_to_stripped_html_when_no_text_part() {
        let mut email = sample_email();
        let mut bv = HashMap::new();
        bv.insert(
            "h".to_string(),
            EmailBodyValue {
                value: "<p>Hello <b>world</b></p>".into(),
                is_encoding_problem: false,
                is_truncated: false,
            },
        );
        email.body_values = Some(bv);
        email.html_body = Some(vec![EmailBodyPart {
            part_id: Some("h".into()),
            blob_id: None,
            size: 25,
            name: None,
            content_type: Some("text/html".into()),
            charset: None,
            disposition: None,
            cid: None,
        }]);

        let v = project_email(email, &Projection::Compact);
        assert_eq!(
            v.get("textBody").and_then(Value::as_str),
            Some("Hello world")
        );
    }

    #[test]
    fn fields_projection_passes_through_and_forces_id() {
        let p = Projection::from_fields_csv("subject,from").unwrap();
        let Projection::Fields(fs) = &p else { panic!() };
        assert_eq!(fs, &vec!["id".to_string(), "subject".into(), "from".into()]);

        let v = project_email(sample_email(), &p);
        let obj = v.as_object().unwrap();
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("subject"));
        assert!(obj.contains_key("from"));
        assert!(!obj.contains_key("threadId"));
        assert!(!obj.contains_key("mailboxIds"));
    }

    #[test]
    fn fields_rejects_unknown_names() {
        let err = Projection::from_fields_csv("subject,unread").unwrap_err();
        assert!(err.contains("unknown field 'unread'"));
    }

    #[test]
    fn fields_rejects_empty() {
        assert!(Projection::from_fields_csv("").is_err());
        assert!(Projection::from_fields_csv(", ,").is_err());
    }

    #[test]
    fn compact_jmap_properties_switches_on_bodies() {
        let p = Projection::Compact;
        let summary = p.jmap_properties(false).unwrap();
        let full = p.jmap_properties(true).unwrap();
        assert!(!summary.contains(&"bodyValues"));
        assert!(full.contains(&"bodyValues"));
    }
}
