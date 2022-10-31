use std::{error::Error, fmt::Display, str::FromStr};

use axum::{headers::HeaderName, http::HeaderValue, http::Request};
use hyper::{header::HOST, Body, StatusCode, Uri};
use regex::Regex;
use sequeda_service_common::X_TENANT_ID_HEADER;

use crate::{
    config::{Authorization, Config, Route},
    openid::User,
};
#[derive(Debug)]
pub struct RequestHandlerError {
    pub status: Option<StatusCode>,
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
                authorizations,
            } = route;
            tracing::info!("adding route with id {id}");

            if route_handlers.iter().any(|r| r.id == id) {
                panic!("duplicate id in config {id}");
            }

            let mut compiled_predicates = vec![];
            let mut compiled_filters = vec![];

            for predicate in predicates.unwrap_or_default() {
                let compiled_predicate = match predicate {
                    crate::config::Predicate::Host(host) => CompiledPredicate::Host(host),
                    crate::config::Predicate::Path(path) => {
                        CompiledPredicate::Path(Regex::new(&path).unwrap())
                    }
                    crate::config::Predicate::Method(method) => CompiledPredicate::Method(method),
                };
                compiled_predicates.push(compiled_predicate);
            }

            for filter in filters.unwrap_or_default() {
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
            let mut compiled_auth = vec![];

            if let Some(authorizations) = authorizations {
                for Authorization {
                    method,
                    has_roles,
                    has_groups,
                } in authorizations
                {
                    compiled_auth.push(CompiledAuthorization {
                        method,
                        has_groups,
                        has_roles,
                    })
                }
            }
            let route = RouteHandler {
                uri,
                id,
                host,
                filters: compiled_filters,
                predicates: compiled_predicates,
                authorizations: compiled_auth,
            };

            tracing::debug!("route `{:?}` added", &route);

            route_handlers.push(route);
        }
        RequestHandler {
            handlers: route_handlers,
        }
    }

    pub async fn handle(
        &self,
        req: &mut Request<Body>,
        user: Option<User>,
    ) -> Result<(), RequestHandlerError> {
        let handler = self
            .handlers
            .iter()
            .find(|h| h.predicates.iter().all(|p| p.match_req(req)));
        let uri = req.uri().clone();

        tracing::debug!(
            "found handler {handler:?} && uri {uri} && path {}",
            uri.path()
        );

        if let Some(handler) = handler {
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
            if !&handler.authorizations.is_empty() {
                let user = match user {
                    Some(user) => user,
                    None => {
                        return Err(RequestHandlerError {
                            status: Some(StatusCode::FORBIDDEN),
                            msg: "Could not retrieve user".to_string(),
                        });
                    }
                };

                for authorization in &handler.authorizations {
                    if !authorization.check_auth(req.method().as_str(), &user) {
                        return Err(RequestHandlerError {
                            status: Some(StatusCode::FORBIDDEN),
                            msg: "Forbidden access".into(),
                        });
                    }
                }
                if let Some(tenant) = user.tenant {
                    req.headers_mut().insert(
                        X_TENANT_ID_HEADER,
                        HeaderValue::from_str(&tenant).map_err(|e| RequestHandlerError {
                            msg: e.to_string(),
                            status: None,
                        })?,
                    );
                }
            }

            let uri = &handler.uri;
            let uri = Uri::try_from(format!("{uri}{path}")).map_err(|e| RequestHandlerError {
                msg: e.to_string(),
                status: None,
            })?;
            tracing::debug!("uri {uri}");
            *req.uri_mut() = uri;
            req.headers_mut().remove(HOST);
            req.headers_mut().insert(HOST, handler.host.clone());

            tracing::debug!("headers {:?}", req.headers());
        } else {
            return Err(RequestHandlerError {
                status: None,
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
    authorizations: Vec<CompiledAuthorization>,
}
#[derive(Debug)]
enum CompiledPredicate {
    Host(String),
    Path(Regex),
    Method(String),
}

#[derive(Debug)]
enum CompiledFilter {
    RewritePath { source: Regex, dest: String },
    AddRequestHeader { key: HeaderName, value: HeaderValue },
    RemoveRequestHeader(HeaderName),
}

#[derive(Debug)]
struct CompiledAuthorization {
    method: String,
    has_roles: Option<Vec<String>>,
    has_groups: Option<Vec<String>>,
}

impl CompiledAuthorization {
    fn check_auth(&self, method: &str, user: &User) -> bool {
        if !self.method.eq_ignore_ascii_case(method) {
            true
        } else {
            let has_roles = if let Some(roles) = &self.has_roles {
                roles.iter().all(|a| user.roles.contains(a))
            } else {
                true
            };
            let has_groups = if let Some(groups) = &self.has_groups {
                groups.iter().all(|a| user.groups.contains(a))
            } else {
                true
            };
            has_roles && has_groups
        }
    }
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
            CompiledPredicate::Method(method) => method.eq_ignore_ascii_case(req.method().as_str()),
        }
    }
}

#[cfg(test)]
mod test {
    use hyper::Uri;
    use regex::Regex;

    #[test]
    fn test_path() {
        let regex = Regex::new("/hello/world/**").unwrap();
        let uri: Uri = "/hello/world".parse().unwrap();

        assert!(regex.is_match(uri.path()));
    }
}
