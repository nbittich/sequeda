use std::{collections::HashMap, env::var, net::SocketAddr, str::FromStr, sync::Arc};

use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Json, Router};
use sequeda_service_common::{setup_tracing, to_value, SERVICE_HOST, SERVICE_PORT};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
struct Country<'a> {
    #[serde(borrow)]
    code: &'a str,
    #[serde(borrow)]
    label: &'a str,
}
impl<'a> Country<'a> {
    const COUNTRIES_STR: &str = include_str!("./countries-codes-filtered.csv");
    fn from_csv() -> Vec<Self> {
        Self::COUNTRIES_STR
            .split('\n')
            .into_iter()
            .skip(1)
            .filter(|line| !line.is_empty())
            .map(|line| line.split(';').collect::<Vec<&'static str>>())
            .map(|csv_row| Self {
                code: csv_row[0],
                label: csv_row[1],
            })
            .collect()
    }
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostalCode<'a> {
    #[serde(borrow)]
    country_code: &'a str,
    #[serde(borrow)]
    postal_code: &'a str,
    #[serde(borrow)]
    name: &'a str,
}

impl<'a> PostalCode<'a> {
    const POST_CODE_STR: &str = include_str!("./geonames-postal-code-filtered.csv");
    fn from_csv() -> HashMap<&'a str, Vec<Self>> {
        Self::POST_CODE_STR
            .split('\n')
            .into_iter()
            .skip(1)
            .filter(|line| !line.is_empty())
            .map(|line| line.split(';').collect::<Vec<&'static str>>())
            .map(|csv_row| {
                (
                    csv_row[0],
                    Self {
                        country_code: csv_row[0],
                        postal_code: csv_row[1],
                        name: csv_row[2],
                    },
                )
            })
            .fold(HashMap::new(), |mut map, (k, v)| {
                map.entry(k).or_insert_with(Vec::new).push(v);
                map
            })
    }
    fn filter_by_country_code(
        postal_codes: &'a HashMap<&'a str, Vec<Self>>,
        code: &'a str,
    ) -> Option<&'a Vec<Self>> {
        postal_codes.get(code)
    }
    #[allow(unused)]
    fn find_by_postal_code(postal_codes: &'a [Self], code: &'a str) -> Option<&'a Self> {
        postal_codes.iter().find(|pc| pc.postal_code == code)
    }
    fn find_by_country_code_and_query(
        postal_codes: &'a HashMap<&'a str, Vec<Self>>,
        country_code: &'a str,
        query: &'a str,
    ) -> Option<Vec<&'a Self>> {
        postal_codes
            .get(country_code)
            .map(|pc| pc.iter())
            .map(|pc| {
                pc.filter(|p| {
                    p.postal_code.starts_with(query)
                        || p.name.to_lowercase().starts_with(&query.to_lowercase())
                })
                .collect()
            })
    }
}

#[tokio::main]
async fn main() {
    setup_tracing();
    let host = var(SERVICE_HOST).unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = var(SERVICE_PORT).unwrap_or_else(|_| String::from("0"));

    let countries = Arc::new(Country::from_csv());
    let postal_codes = Arc::new(PostalCode::from_csv());

    let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();
    tracing::info!("listening on {:?}", addr);
    let app = Router::new()
        .route("/municipality/by-country", get(find_by_country))
        .route("/municipality/by-query", get(find_by_query))
        .route("/countries", get(get_countries))
        .layer(Extension(postal_codes))
        .layer(Extension(countries));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn find_by_country(
    Extension(postal_codes): Extension<Arc<HashMap<&'static str, Vec<PostalCode<'_>>>>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    tracing::debug!("Find by country route entered!");
    Json(to_value(PostalCode::filter_by_country_code(
        &postal_codes,
        &params["country_code"],
    )))
}

async fn get_countries(
    Extension(countries): Extension<Arc<Vec<Country<'_>>>>,
) -> impl IntoResponse {
    tracing::debug!("Get countries route entered!");
    Json(to_value(&*countries))
}

async fn find_by_query(
    Extension(postal_codes): Extension<Arc<HashMap<&'static str, Vec<PostalCode<'_>>>>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    tracing::debug!("Find by query route entered!");
    Json(to_value(PostalCode::find_by_country_code_and_query(
        &postal_codes,
        &params["country_code"],
        &params["query"],
    )))
}

#[cfg(test)]
mod test {
    use crate::{Country, PostalCode};

    #[test]
    fn countries_test() {
        let countries = Country::from_csv();
        let postal_codes = PostalCode::from_csv();
        let Some(belgium) = PostalCode::filter_by_country_code(&postal_codes, "BE") else {panic!("belgium not found")};
        let Some(uk) = PostalCode::filter_by_country_code(&postal_codes, "GB") else {panic!("united kingdom not found")};
        assert_eq!(
            PostalCode::find_by_postal_code(&belgium, "1083"),
            Some(&PostalCode {
                country_code: "BE",
                postal_code: "1083",
                name: "Ganshoren"
            })
        );
        assert_eq!(belgium.len(), 2781);
        assert_eq!(uk.len(), 27430);
        assert_eq!(
            countries.iter().find(|c| c.code == "BE"),
            Some(&Country {
                code: "BE",
                label: "Belgium"
            })
        );

        assert_eq!(
            PostalCode::find_by_country_code_and_query(&postal_codes, "BE", "108"),
            Some(vec![
                &PostalCode {
                    country_code: "BE",
                    postal_code: "1080",
                    name: "Molenbeek-Saint-Jean",
                },
                &PostalCode {
                    country_code: "BE",
                    postal_code: "1081",
                    name: "Koekelberg",
                },
                &PostalCode {
                    country_code: "BE",
                    postal_code: "1082",
                    name: "Berchem-Sainte-Agathe",
                },
                &PostalCode {
                    country_code: "BE",
                    postal_code: "1083",
                    name: "Ganshoren",
                },
            ],)
        );
    }
}
