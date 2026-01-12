use mongodb::{Collection, Database, bson};
use rand::{Rng, distr::Alphanumeric, rng};

use super::error::Error;
use crate::model;

const COLLECTION: &str = "urls";
const LENGTH: usize = 6;

#[derive(Clone)]
pub struct Url {
    collection: Collection<model::Url>,
}

impl Url {
    pub fn new(db: Database) -> Self {
        Url {
            collection: db.collection(COLLECTION),
        }
    }

    pub fn random_key() -> String {
        rng()
            .sample_iter(&Alphanumeric)
            .take(LENGTH)
            .map(char::from)
            .collect()
    }

    pub async fn fetch(&self, name: &str) -> Option<model::Url> {
        self.collection
            .find_one(bson::doc! { "key": name })
            .await
            .ok()
            .flatten()
    }

    pub async fn store(&self, url: &model::Url) -> Result<(), Error> {
        self.collection
            .insert_one(url)
            .await
            .map_err(|err| Error {
                error: Box::new(err),
            })
            .map(|_| ())
    }
}

mod tests {
    #[test]
    fn random_key() {
        let s1 = super::Url::random_key();
        assert_eq!(s1.len(), 6);

        let s2 = super::Url::random_key();

        assert_ne!(s1, s2);
    }
}
