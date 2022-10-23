use std::{error::Error, fmt::Display, str::FromStr};

use axum::{headers::HeaderName, http::HeaderValue};
use hyper::{header::HOST, Body, Request, Uri};
use regex::Regex;

use crate::config::{Config, Route};
#[derive(Debug)]
pub struct RequestHandlerError {
    msg: String,
}

impl Error for RequestHandlerError {}

impl Display for RequestHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
#[derive(Debug)]
pub struct RequestHandler {
    handlers: Vec<RouteHandler>,
}

impl RequestHandler {
    pub fn from_config(config: Config) -> Self {
        let mut route_handlers: Vec<RouteHandler> = vec![];
        for route in config.routes {
            let Route {
                id,
                uri,
                predicates,
                filters,
            } = route;
            tracing::info!("adding route with id {id}");

            if route_handlers.iter().any(|r| r.id == id) {
                panic!("duplicate id in config {id}");
            }

            let mut compiled_predicates = vec![];
            let mut compiled_filters = vec![];

            for predicate in predicates {
                let compiled_predicate = match predicate {
                    crate::config::Predicate::Host(host) => CompiledPredicate::Host(host),
                    crate::config::Predicate::Path(path) => {
                        CompiledPredicate::Path(Regex::new(&path).unwrap())
                    }
                };
                compiled_predicates.push(compiled_predicate);
            }

            for filter in filters {
                let compiled_filter = match filter {
                    crate::config::Filter::RewritePath { source, dest } => {
                        CompiledFilter::RewritePath {
                            source: Regex::new(&source).unwrap(),
                            dest,
                        }
                    }
                    crate::config::Filter::AddRequestHeader { key, value } => {
                        CompiledFilter::AddRequestHeader {
                            key: HeaderName::from_str(&key).unwrap(),
                            value: HeaderValue::from_str(&value).unwrap(),
                        }
                    }
                    crate::config::Filter::RemoveRequestHeader(header) => {
                        CompiledFilter::RemoveRequestHeader(HeaderName::from_str(&header).unwrap())
                    }
                };
                compiled_filters.push(compiled_filter);
            }

            let host = {
                let iri = Uri::try_from(&uri).unwrap();
                HeaderValue::from_str(iri.host().unwrap()).unwrap()
            };
            let route = RouteHandler {
                uri,
                id,
                host,
                filters: compiled_filters,
                predicates: compiled_predicates,
            };
            tracing::debug!("route `{:?}` added", &route);

            route_handlers.push(route);
        }
        RequestHandler {
            handlers: route_handlers,
        }
    }

    pub fn handle(&self, req: &mut Request<Body>) -> Result<(), RequestHandlerError> {
        let handler = self
            .handlers
            .iter()
            .find(|h| h.predicates.iter().any(|p| p.match_req(req)));

        if let Some(handler) = handler {
            let uri = req.uri().clone();
            let mut path = uri
                .path_and_query()
                .map(|v| v.as_str())
                .unwrap_or_else(|| uri.path())
                .to_string();
            for filter in &handler.filters {
                match filter {
                    CompiledFilter::RewritePath { source, dest } => {
                        path = source.replace(&path, dest).to_string();
                    }
                    CompiledFilter::AddRequestHeader { key, value } => {
                        let headers = req.headers_mut();
                        headers.append(key, value.clone());
                    }
                    CompiledFilter::RemoveRequestHeader(header_name) => {
                        let headers = req.headers_mut();
                        headers.remove(header_name);
                    }
                }
            }
            let uri = &handler.uri;
            let uri = Uri::try_from(format!("{uri}{path}"))
                .map_err(|e| RequestHandlerError { msg: e.to_string() })?;
            tracing::debug!("uri {uri}");
            *req.uri_mut() = uri;
            req.headers_mut().remove(HOST);
            req.headers_mut().insert(HOST, handler.host.clone());

            tracing::debug!("headers {:?}", req.headers());
        } else {
            return Err(RequestHandlerError {
                msg: format!("Could not find an handler for that uri {}", req.uri()),
            });
        }

        Ok(())
    }
}

#[derive(Debug)]
struct RouteHandler {
    id: String,
    uri: String,
    host: HeaderValue,
    predicates: Vec<CompiledPredicate>,
    filters: Vec<CompiledFilter>,
}
#[derive(Debug)]
enum CompiledPredicate {
    Host(String),
    Path(Regex),
}
#[derive(Debug)]
enum CompiledFilter {
    RewritePath { source: Regex, dest: String },
    AddRequestHeader { key: HeaderName, value: HeaderValue },
    RemoveRequestHeader(HeaderName),
}
impl CompiledPredicate {
    fn match_req(&self, req: &Request<Body>) -> bool {
        let uri = req.uri();
        match self {
            CompiledPredicate::Host(host) => {
                let host_from_req = uri
                    .host()
                    .or_else(|| req.headers().get(HOST).and_then(|h| h.to_str().ok()));
                tracing::debug!("{host}, {:?}", &host_from_req);
                host_from_req.eq(&Some(host))
            }
            CompiledPredicate::Path(path_re) => path_re.is_match(uri.path()),
        }
    }
}
