use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Eq, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Route {
    pub id: String,
    pub uri: String,
    pub predicates: Option<Vec<Predicate>>,
    pub filters: Option<Vec<Filter>>,
    pub authorizations: Option<Vec<Authorization>>,
}

#[derive(Deserialize, PartialEq, Eq, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Authorization {
    pub method: String,
    pub has_roles: Option<Vec<String>>,
    pub has_groups: Option<Vec<String>>,
}

#[derive(Deserialize, PartialEq, Eq, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub order: usize,
    pub routes: Vec<Route>,
}

#[derive(Deserialize, PartialEq, Eq, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Predicate {
    Host(String),
    Path(String),
    Method(String),
}
#[derive(Deserialize, PartialEq, Eq, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    RewritePath { source: String, dest: String },
    AddRequestHeader { key: String, value: String },
    RemoveRequestHeader(String),
}

impl Config {
    #[allow(unused)]
    pub fn deserialize(yaml: &str) -> Self {
        match serde_yml::from_str(yaml) {
            Ok(yaml) => yaml,
            Err(e) => panic!("could not deserialize yaml {e}"),
        }
    }
    pub fn merge(mut self, config: &mut Config) -> Self {
        self.routes.append(&mut config.routes);
        self
    }
    pub fn from_file(config_path: &Path) -> Self {
        let file = File::open(config_path).unwrap();
        let buf_reader = BufReader::new(file);
        serde_yml::from_reader(buf_reader).unwrap()
    }

    pub fn from_dir(service_volume_path: &Path) -> Self {
        if !service_volume_path.is_dir() {
            panic!("{service_volume_path:?} not a directory!");
        }
        let mut configs: Vec<Config> = service_volume_path
            .read_dir()
            .expect("could not read config directory!")
            .filter_map(|e| e.ok().map(|p| p.path()).filter(|p| p.is_file()))
            .inspect(|p| tracing::debug!("loading config {p:?}"))
            .map(|p| Self::from_file(p.as_path()))
            .collect();

        configs.sort_by_key(|c| c.order);

        let config: Option<Config> = configs.into_iter().reduce(|acc, mut e| acc.merge(&mut e));

        match config {
            Some(config) => config,
            None => panic!("Could not build config from volume!"),
        }
    }

    #[allow(unused)]
    pub fn serialize(&self) -> String {
        match serde_yml::to_string(&self) {
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
         order: 0
         routes:
            - id: yahoo_finance_chart
              uri: https://query1.finance.yahoo.com
              predicates:
              - !path /proxy/yahoo-finance/chart/**
              filters:
              - !rewrite_path
                 source: /proxy/yahoo-finance/chart/(?P<segment>.*)
                 dest: /v8/finance/chart/${segment}
            - id: auth
              uri: http://auth.somehost.org:8080
              predicates:
              - !host auth.somehost.org
              filters:
              - !add_request_header
                 key: "X-Forwarded-Port"
                 value: "443"
            - id: auth2
              uri: http://auth2.somehost.org:8080
              predicates:
              - !host auth2.somehost.org
              filters:
              - !remove_request_header "X-Forwarded-Port"
        "#;

        let config = Config::deserialize(config);
        assert_eq!(
            config,
            Config {
                order: 0,
                routes: vec![
                    Route {
                        id: "yahoo_finance_chart".into(),
                        uri: "https://query1.finance.yahoo.com".into(),
                        predicates: Some(vec![Predicate::Path(
                            "/proxy/yahoo-finance/chart/**".into()
                        )]),
                        filters: Some(vec![Filter::RewritePath {
                            source: "/proxy/yahoo-finance/chart/(?P<segment>.*)".into(),
                            dest: "/v8/finance/chart/${segment}".into(),
                        }]),
                        authorizations: None,
                    },
                    Route {
                        id: "auth".into(),
                        uri: "http://auth.somehost.org:8080".into(),
                        predicates: Some(vec![Predicate::Host("auth.somehost.org".into())]),
                        filters: Some(vec![Filter::AddRequestHeader {
                            key: "X-Forwarded-Port".into(),
                            value: "443".into()
                        }]),
                        authorizations: None,
                    },
                    Route {
                        id: "auth2".into(),
                        uri: "http://auth2.somehost.org:8080".into(),
                        predicates: Some(vec![Predicate::Host("auth2.somehost.org".into())]),
                        filters: Some(vec![Filter::RemoveRequestHeader("X-Forwarded-Port".into())]),
                        authorizations: None,
                    }
                ],
            }
        );
    }
}
