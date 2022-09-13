# Getting started

## Setting up the backend

1. cd to the backend directory
2. Create an empty sqlite db: `touch hotsauce.db`
3. Install sea-orm-cli: `cargo install sea-orm-cli`
4. Setup db schema: `DATABASE_URL=sqlite://hotsauce.db sea-orm-cli migrate refresh`
5. Startup the api server: `cargo run --bin api`

## Updating the database schema

1. Create a new migration in backend/migration/src
2. Ensure the DATABASE_URL env var is set: `export DATABASE_URL=sqlite://hotsauce.db`
3. Either:
- `sea-orm-cli migrate refresh` to drop all tables and recreate with new schema
- `sea-orm-cli migrate up` to just apply the latest migration(s)
4. Update the entity code
- `sea-orm-cli generate entity --with-serde both --output-dir backend/entity/src`
- `rm backend/entity/src/mod.rs`
- Review the diffs in the generated entity code
    - In particular at the time of writing, sea-orm-cli will create a mod.rs that we don't need, and for sqlite based dev databases will attempt to use `String`s for datetimes rather than well-typed chrono `DateTimeUtc`s

## Setup the frontend dev server

1. cd to the frontend directory
2. `npm install`
3. `npm run dev`