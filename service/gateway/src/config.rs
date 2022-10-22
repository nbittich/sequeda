use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Eq, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Route {
    pub id: String,
    pub uri: String,
    pub predicates: Vec<Predicate>,
    pub filters: Vec<Filter>,
}

#[derive(Deserialize, PartialEq, Eq, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub routes: Vec<Route>,
}

#[derive(Deserialize, PartialEq, Eq, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Predicate {
    Host(String),
    Path(String),
}
#[derive(Deserialize, PartialEq, Eq, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    RewritePath { source: String, dest: String },
    AddRequestHeader { key: String, value: String },
}

impl Config {
    #[allow(unused)]
    pub fn deserialize(toml: &str) -> Self {
        match serde_yaml::from_str(toml) {
            Ok(toml) => toml,
            Err(e) => panic!("could not deserialize toml {e}"),
        }
    }
    pub fn deserialize_file(config_path: &Path) -> Self {
        let file = File::open(config_path).unwrap();
        let buf_reader = BufReader::new(file);
        serde_yaml::from_reader(buf_reader).unwrap()
    }
    #[allow(unused)]
    pub fn serialize(&self) -> String {
        match serde_yaml::to_string(&self) {
            Ok(toml) => toml,
            Err(e) => panic!("could not serialize toml {e}"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::{Filter, Predicate, Route};

    use super::Config;

    #[test]
    fn test_deserialize() {
        let config = r#"
         routes:
            - id: yahoo_finance_chart
              uri: https://query1.finance.yahoo.com
              predicates:
              - !path /proxy/yahoo-finance/chart/**
              filters:
              - !rewrite_path
                 source: /proxy/yahoo-finance/chart/(?P<segment>.*)
                 dest: /v8/finance/chart/${segment}
              - !add_request_header
                 key: "X-Forwarded-Port"
                 value: "443"
        "#;

        let config = Config::deserialize(config);
        assert_eq!(
            config,
            Config {
                routes: vec![Route {
                    id: "yahoo_finance_chart".into(),
                    uri: "https://query1.finance.yahoo.com".into(),
                    predicates: vec![Predicate::Path("/proxy/yahoo-finance/chart/**".into())],
                    filters: vec![
                        Filter::RewritePath {
                            source: "/proxy/yahoo-finance/chart/(?P<segment>.*)".into(),
                            dest: "/v8/finance/chart/${segment}".into(),
                        },
                        Filter::AddRequestHeader {
                            key: "X-Forwarded-Port".into(),
                            value: "443".into()
                        }
                    ],
                }],
            }
        );
    }
}
