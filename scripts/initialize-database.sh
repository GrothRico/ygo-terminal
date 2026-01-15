#/usr/bin/env sh

if [ ! $(which docker) ]; then
    echo "Need to install docker. Stopping initialization."
    exit 1
fi

if [ ! $(which jq) ]; then
    echo "Need to install jq. Stopping initialization."
    exit 1
fi

if [ ! -f "var/all-cards.json" ]; then
    mkdir -p "var"
    echo "Downloading data for all cards"
    echo "================================================"
    curl "https://db.ygoprodeck.com/api/v7/cardinfo.php?language=de" | jq '.data' > var/all-cards.json
fi
if [ ! -f "var/goat.json" ]; then
    mkdir -p "var"
    echo "Downloading data for GOAT-format cards"
    echo "================================================"
    curl "https://db.ygoprodeck.com/api/v7/cardinfo.php?language=de&format=goat" | jq '.data' > var/goat.json
fi
echo "Start up MongoDB container"
echo "================================================"
docker compose up -d
echo "Wait for MongoDB container to become ready"
echo "================================================"
sleep 5
echo "Import data for all cards to MongoDB container"
echo "================================================"
cat var/all-cards.json | docker exec -i ygo-terminal-ygo-card-db-1 mongoimport \
  --db db --collection allCards --jsonArray --uri "mongodb://root:root@127.0.0.1:27017/db?authSource=admin"
sleep 5
echo "Import data for GOAT-format cards to MongoDB container"
echo "================================================"
cat var/goat.json | docker exec -i ygo-terminal-ygo-card-db-1 mongoimport \
  --db db --collection goat --jsonArray --uri "mongodb://root:root@127.0.0.1:27017/db?authSource=admin"
