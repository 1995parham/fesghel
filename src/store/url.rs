use mongodb::Database;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::model;
use super::error::Error;

const COLLECTION: &str = "urls";
const LENGTH: usize = 6;

#[derive(Clone)]
pub struct URL {
    db: Database,
}

impl URL {
    pub fn new(db: Database) -> Self {
        URL {
            db,
        }
    }

    pub fn random_key() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(LENGTH).collect()
    }

    pub async fn fetch(&self, name: &str) -> Option<model::URL> {
        let res = self.db.collection(COLLECTION).find_one(bson::doc! { "key": name }, None).await;

        match res {
            Ok(d) =>  {
                match d {
                    Some(d) => bson::from_bson(bson::Bson::Document(d)).expect("from_bson failed"),
                    None => None,
                }
            },
            Err(..) => None
        }
    }

    pub async fn store(&self, url: &model::URL) ->  Result<(), Error> {
        match bson::to_bson(url).expect("to_bson failed") {
            bson::Bson::Document(doc) =>
                self.db.collection(COLLECTION).insert_one(doc, None).await.map_err(|err| Error{error: Box::new(err)}).map(|_| ()),
            _ => Ok(()),
        }
    }
}

mod tests {
    #[test]
    fn random_key() {
        let s1 = super::URL::random_key();
        assert_eq!(s1.len(), 6);

        let s2 = super::URL::random_key();

        assert_ne!(s1, s2);
    }
}
