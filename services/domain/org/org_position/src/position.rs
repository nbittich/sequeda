use chrono::{Local, NaiveDateTime};
use sequeda_service_common::IdGenerator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub level: Option<PositionLevel>,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionUpsert {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub level: Option<PositionLevel>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            id: IdGenerator.get(),
            name: Default::default(),
            description: Default::default(),
            level: Some(PositionLevel::Operational),
            creation_date: Local::now().naive_local(),
            updated_date: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionLevel {
    // highest possible level
    Executive,
    //Operational-level positions present a valuable leg up into any given
    // industry for entry-level or new employee
    Operational,
    Management,
    Junior,
    Medior,
    Senior,
}

#[cfg(test)]
mod test {
    use super::Position;

    #[test]
    fn test_deserialize() {
        let input = r#"
        {
            "_id": "639f6afd42bfaeed62a87a98",
            "creationDate": "2022-12-18T19:33:17.752Z",
            "name": "Chief executive office",
            "level": "EXECUTIVE"
        }
        "#;

        let pos: Position = serde_json::from_str(input).unwrap();
        dbg!(pos);
    }
}
