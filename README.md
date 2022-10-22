# Sequeda

Experiments for openartcoded next.

Currently, the stack requires a minimum of 3.5Gb of ram. 

We can do better and make the entire stack run on a smaller machine.

A first goal would be to replace java (almost) entirely, and keep only the relevant other services (such as keycloak, mongodb & redis).

## Message Broker

- Dead simple message queue system
- Goal is to replace artemis (java) with only the relevant parts, such as persistence of the messages sent (until someone consumes it)

## Store

- High level abstraction of mongodb. Should replace spring-data

## Message Client

- Simple client to consume / produce messages.

## Gateway

- a replacement for spring cloud gateway


## Docker usage

`docker build -t sequeda/gateway --build-arg CRATE_NAME=sequeda_gateway . # replace arg by the target crate`

`docker run -v /tmp:/tmp -p 8080:8080 -e SERVICE_HOST=0.0.0.0 -e SERVICE_PORT=8080  -it sequeda/gateway`