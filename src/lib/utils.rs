use serde::Deserialize;

pub fn parse_enum_optional<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + std::str::FromStr,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;

    if let Some(value) = opt {
        match value.parse::<T>() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}
