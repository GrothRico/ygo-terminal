#!/usr/bin/env sh

if [ ! $(which docker) ]; then
    echo "Need to install docker. Stopping initialization."
    exit 1
fi

if [ ! $(which jq) ]; then
    echo "Need to install jq. Stopping initialization."
    exit 1
fi

if [ ! -f "var/data.json" ]; then
    mkdir -p "var"
    echo "Downloading data"
    echo "================================================"
    curl "https://db.ygoprodeck.com/api/v7/cardinfo.php?language=de" | jq '.data' > var/data.json
fi
echo "Start up MongoDB container"
echo "================================================"
docker compose up -d
echo "Wait for MongoDB container to become ready"
echo "================================================"
sleep 5
echo "Import card data to MongoDB container"
echo "================================================"
cat var/data.json | docker exec -i ygo-terminal-ygo-card-db-1 mongoimport \
  --db db --collection cards --jsonArray --uri "mongodb://root:root@127.0.0.1:27017/db?authSource=admin"
