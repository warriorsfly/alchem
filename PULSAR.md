docker run -dit --rm \
--name pulsar-postgres \
-p 5432:5432 \
-e POSTGRES_PASSWORD=123456 \
postgres:alpine -c wal_level=logical

docker run -dit \
    --name pulsar-standalone \
    -p 6650:6650  -p 8080:8080 \
    --mount source=pulsardata,target=/pulsar/data \
    --mount source=pulsarconf,target=/pulsar/conf \
    apachepulsar/pulsar bin/pulsar standalone



    docker cp pulsar-io-debezium-postgres-2.10.0.nar 4546629af02ceba484073c10f8bb6b9176d1a71f0581646bd369c1a6a7d5874c:/pulsar/connectors/pulsar-io-debezium-postgres-2.10.0.nar