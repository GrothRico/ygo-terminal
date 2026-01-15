use mongodb::{
    Client, Collection,
    bson::{Document, doc},
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "mongodb://root:root@localhost:27017";
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("db");
    let goat_collection: Collection<Document> = database.collection("goat");
    let _all_cards_collection: Collection<Document> = database.collection("allCards");

    let find_one_res = goat_collection
        .find_one(doc! {"name": String::from("Dunkler Magier")})
        .await?;
    if let Some(card) = find_one_res {
        let sets = card.get("card_sets").unwrap().as_array().unwrap();
        let x: Vec<&str> = sets
            .iter()
            .map(|set| {
                set.as_document()
                    .unwrap()
                    .get("set_code")
                    .unwrap()
                    .as_str()
                    .unwrap()
            })
            .collect();
        println!("{:?}", x);
    }

    Ok(())
}
