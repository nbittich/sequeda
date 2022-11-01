# Api Gateway

Simple api gateway with built-in openid connect authentication / authorization.

Use a similar configuration strategy than spring cloud gateway, but with configurable authorizations, e.g:

```yaml
routes:
  - id: yahoo_finance_chart
    uri: https://query1.finance.yahoo.com
    predicates:
      - !host api.somehost.org
      - !path /proxy/yahoo-finance/chart/**
    filters:
      - !rewrite_path
        source: /proxy/yahoo-finance/chart/(?P<segment>.*)
        dest: /v8/finance/chart/${segment}
      - !remove_request_header Cookie
    authorizations:
      - !authorization
        method: "GET"
        has_roles:
          - creep
        has_groups:
          - /yahoo
          - /manager
  - id: person_find_all
    uri: http://person
    predicates:
      - !path /person/find-all/**
    authorizations:
      - !authorization
        method: "GET"
        has_roles:
          - creep
    filters:
      - !rewrite_path
        source: /person/find-all/**
        dest: /find-all
  - id: person_upsert
    uri: http://person
    filters:
      - !rewrite_path
        source: /person/**
        dest: /
    predicates:
      - !path /person/**
      - !method POST
    authorizations:
      - !authorization
        method: "POST"
        has_roles:
          - creep

```

## Setup

```yaml
  ####### Redis for SessionStore #######
  redis:
    image: redis:alpine
    restart: always
    networks:
      sequeda:
    command: redis-server  --appendonly yes
    volumes:
      - ./data/redis:/data
  ####### Gateway #######
  gateway:
    build:
      context: ../
      args:
        CRATE_NAME: sequeda_gateway
    volumes:
      - ./config/gateway/config.yml:/config/gateway.yml
    environment:
      RUST_LOG: DEBUG
      SERVICE_CONFIG_VOLUME: /config
      SERVICE_HOST: 0.0.0.0
      SERVICE_PORT: 80
      OPENID_ENABLED: "true"
      APP_ROOT_URL: "https://api.somehost.org"
      OPENID_ISSUER_URL: "http://auth.somehost.org/realms/sequeda"
      OPENID_CLIENT_ID: "sequeda-auth"
      OPENID_CLIENT_SECRET: "X9quJGCCpBISQt6uNs62gkPeYd4g2gsp"
      SESSION_REDIS_URL: "redis://redis"
    restart: "always"
    networks:
      sequeda:
```