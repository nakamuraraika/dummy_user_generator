
use fake::faker::chrono::raw::*;
use fake::faker::internet::raw::FreeEmail;
use fake::locales::EN;
use fake::uuid::UUIDv4;
use fake::{Dummy, Faker, Fake};
use fake::faker::name::raw::*;
use chrono::prelude::*;
use chrono::Utc;

#[derive(Debug, serde::Serialize, Dummy)]
pub struct User {
    #[dummy(faker = "UUIDv4")]
    pub id: String,
    #[dummy(faker = "FirstName(EN)")]
    pub first_name: String,
    #[dummy(faker = "LastName(EN)")]
    pub last_name: String,
    #[dummy(faker = "1..2")]
    pub gender: usize,
    #[dummy(faker = "DateTimeBetween(EN, Utc.with_ymd_and_hms(1995, 1, 1, 0, 0, 0).unwrap(), Utc::now())")]
    pub birth_day: String,
    #[dummy(faker = "FreeEmail(EN)")]
    pub email: String,
    #[dummy(faker = "UUIDv4")]
    pub uuid: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut wtr = csv::Writer::from_path("dummies.csv")?;

    for n in 0..10_000_000 {
        let user: User = Faker.fake();
        wtr.serialize(&user)?;

        if n % 100_000 == 0 {
            println!("{} users generated", n);
        }
    }

    wtr.flush()?;

    Ok(())
}