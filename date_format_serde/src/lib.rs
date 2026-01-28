pub mod date_format_naive {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%m/%d/%y";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestDate {
        #[serde(with = "date_format_naive")]
        date: NaiveDate,
    }

    #[test]
    fn test_serialize_deserialize() {
        let test_date = TestDate {
            date: NaiveDate::from_ymd_opt(2024, 12, 25).unwrap(),
        };

        let json = serde_json::to_string(&test_date).unwrap();
        assert_eq!(json, r#"{"date":"12/25/24"}"#);

        let deserialized: TestDate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, test_date);
    }
}