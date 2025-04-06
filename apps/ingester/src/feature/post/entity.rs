use chrono::{DateTime, TimeZone, Utc};
use clickhouse::Row;
use prost_types::Timestamp;
use proto::posts::Post as ProtoPost;
use serde::Deserialize;
#[derive(Debug, Row, Deserialize)]
pub struct PostStruct {
    pub chat_id: i64,
    pub message_id: u32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub post_timestamp: chrono::DateTime<chrono::Utc>,
    pub text: String,
}

impl From<ProtoPost> for PostStruct {
    fn from(proto: ProtoPost) -> Self {
        fn timestamp_to_datetime(ts: Option<&Timestamp>) -> DateTime<Utc> {
            ts.map(|t| {
                let millis_nanos = (t.nanos / 1_000_000) * 1_000_000;

                Utc.timestamp_opt(t.seconds, millis_nanos as u32)
                    .single()
                    .unwrap_or_else(|| Utc::now())
            })
            .unwrap_or_else(|| {
                let now = Utc::now();
                let millis_nanos = (now.timestamp_subsec_nanos() / 1_000_000) * 1_000_000;

                Utc.timestamp_opt(now.timestamp(), millis_nanos)
                    .single()
                    .unwrap_or(now)
            })
        }

        PostStruct {
            chat_id: proto.chat_id,
            message_id: proto.message_id,
            timestamp: timestamp_to_datetime(proto.timestamp.as_ref()),
            post_timestamp: timestamp_to_datetime(proto.post_timestamp.as_ref()),
            text: proto.text,
        }
    }
}
