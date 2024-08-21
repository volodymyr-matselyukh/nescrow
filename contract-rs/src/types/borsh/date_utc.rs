use borsh::{BorshDeserialize, BorshSerialize};
use chrono::{DateTime, TimeZone, Utc};

pub struct UtcDateTime(pub DateTime<Utc>);

impl BorshSerialize for UtcDateTime {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let timestamp = self.0.timestamp();
        timestamp.serialize(writer)
    }
}

impl BorshDeserialize for UtcDateTime {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let timestamp = i64::deserialize_reader(reader)?;
        Ok(UtcDateTime(
            Utc.timestamp_opt(timestamp, 0)
                .single()
                .expect("Invalid timestamp"),
        ))
    }
}
