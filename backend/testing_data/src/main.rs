use anyhow::Result;
use chrono::{TimeZone, Utc};
use entity::{manufacturer, prelude::Manufacturer};
use rand::prelude::*;
use sea_orm::{prelude::*, Database, QuerySelect, Set};

const MANUFACTURER_NAMES: [&str; 4] = ["Mahi", "Wiltshire Chilli Farm", "Melinda's", "Torchbearer"];

const SAUCES: [(&str, &str); 13] = [
    ("Mahi", "Scorpion Pepper and Passion Fruit"),
    ("Mahi", "Bhut Jolokia"),
    ("Mahi", "Peri Peri Hot"),
    ("Mahi", "Green Savina Hebanero Sweet Heat Sauce"),
    ("Wiltshire Chilli Farm", "Reaper Habanero"),
    ("Wiltshire Chilli Farm", "Golden Bonnet"),
    ("Wiltshire Chilli Farm", "Regret"),
    ("Torchbearer", "Nekrogoblikon's Goblin Sauce"),
    ("Torchbearer", "Garlic Reaper"),
    ("Torchbearer", "Headless Horseradish"),
    ("Melinda's", "Original Habanero XXXtra Hot Pepper Sauce"),
    ("Melinda's", "Amarillo Habanero Hot Mustard Sauce"),
    ("Melinda's", "Chipotle Pepper Hot Sauce"),
];

const AXES: [(&str, &str, &str); 4] = [
    ("Overall", "Would actively avoid trying again", "The best"),
    ("Heat", "No heat at all", "Insanely hot"),
    (
        "Flavour",
        "Actively unpleasant flavour",
        "Divinely delicious",
    ),
    (
        "Versatility",
        "Highly situational",
        "Slather it on everything",
    ),
];

// Courtesy of https://usernamegenerator.com/random
const USERNAMES: [&str; 22] = [
    "MousePepper",
    "SirPepper",
    "ParadoxPepper",
    "PepperStrife",
    "SauceTraitor",
    "SauceWiggle",
    "SauceFactor",
    "SaucePolice",
    "SauceDwarf",
    "SauceEvil",
    "RadioSauce",
    "ModestSauce",
    "HashtagSauce",
    "SauceRifle",
    "AbusiveSauce",
    "ChilliSake",
    "WinningChilli",
    "ChilliBanter",
    "CrankyChilli",
    "ChilliMother",
    "ChilliSawyer",
    "ChilliPharaoh",
];

async fn insert_manufacturers(db: &DatabaseConnection) -> Result<()> {
    for name in MANUFACTURER_NAMES {
        entity::manufacturer::ActiveModel {
            name: Set(name.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }

    Ok(())
}

async fn insert_sauces(db: &DatabaseConnection) -> Result<()> {
    for (m, s) in SAUCES {
        let manufacturer_id = Manufacturer::find()
            .filter(manufacturer::Column::Name.eq(m))
            .one(db)
            .await?
            .unwrap()
            .id;

        entity::sauce::ActiveModel {
            name: Set(s.to_string()),
            manufacturer: Set(manufacturer_id),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }

    Ok(())
}

async fn insert_rating_axes(db: &DatabaseConnection) -> Result<()> {
    for (name, min_desc, max_desc) in AXES {
        entity::rating_axis::ActiveModel {
            name: Set(name.to_string()),
            min_value: Set(0.0),
            max_value: Set(10.0),
            min_value_desc: Set(min_desc.to_string()),
            max_value_desc: Set(max_desc.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }

    Ok(())
}

async fn insert_users(db: &DatabaseConnection) -> Result<()> {
    for name in USERNAMES {
        entity::user::ActiveModel {
            username: Set(name.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }

    Ok(())
}

async fn insert_random_review(db: &DatabaseConnection) -> Result<()> {
    let mut rng = thread_rng();

    let username = *USERNAMES.choose(&mut rng).unwrap();
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(username))
        .column(entity::user::Column::Id)
        .one(db)
        .await?
        .unwrap()
        .id;

    let (manufacturer_name, sauce_name) = *SAUCES.choose(&mut rng).unwrap();
    let sauce = entity::sauce::Entity::find()
        .inner_join(entity::manufacturer::Entity)
        .filter(entity::sauce::Column::Name.eq(sauce_name))
        .filter(entity::manufacturer::Column::Name.eq(manufacturer_name))
        .one(db)
        .await?
        .unwrap()
        .id;

    let epoch = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0);
    let timestamp = epoch + chrono::Duration::seconds(rng.gen_range(0..(3600 * 24 * 365)));

    let text_length = rng.gen_range(5..50);
    let text = Some(lipsum::lipsum_words(text_length));

    entity::review::ActiveModel {
        sauce: Set(sauce),
        user: Set(user),
        timestamp: Set(timestamp),
        text: Set(text),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let db = Database::connect("sqlite://hotsauce.db").await?;

    insert_manufacturers(&db).await?;
    insert_sauces(&db).await?;
    insert_rating_axes(&db).await?;
    insert_users(&db).await?;

    for _ in 0..100 {
        insert_random_review(&db).await?;
    }

    Ok(())
}
