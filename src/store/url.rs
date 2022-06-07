use mongodb::Database;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use super::error::Error;
use crate::model;

const COLLECTION: &str = "urls";
const LENGTH: usize = 6;

#[derive(Clone)]
pub struct Url {
    db: Database,
}

impl Url {
    pub fn new(db: Database) -> Self {
        Url { db }
    }

    pub fn random_key() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(LENGTH)
            .map(char::from)
            .collect()
    }

    pub async fn fetch(&self, name: &str) -> Option<model::Url> {
        let res = self
            .db
            .collection(COLLECTION)
            .find_one(bson::doc! { "key": name }, None)
            .await;

        match res {
            Ok(d) => match d {
                Some(d) => bson::from_bson(bson::Bson::Document(d)).expect("from_bson failed"),
                None => None,
            },
            Err(..) => None,
        }
    }

    pub async fn store(&self, url: &model::Url) -> Result<(), Error> {
        match bson::to_bson(url).expect("to_bson failed") {
            bson::Bson::Document(doc) => self
                .db
                .collection(COLLECTION)
                .insert_one(doc, None)
                .await
                .map_err(|err| Error {
                    error: Box::new(err),
                })
                .map(|_| ()),
            _ => Ok(()),
        }
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
