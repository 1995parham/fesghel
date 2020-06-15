use mongodb::Database;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::model;

const COLLECTION: &str = "urls";

pub struct URL {
    db: Database,
}

impl Clone for URL {
    fn clone(&self) -> Self {
        URL{
            db: self.db.clone(),
        }
    }
}

impl URL {
    pub fn new(db: Database) -> Self {
        URL {
            db,
        }
    }

    pub fn random_key() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(6).collect()
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

    pub async fn store(&self, url: &model::URL) {
        match bson::to_bson(url).expect("to_bson failed") {
            bson::Bson::Document(doc) => {
                self.db.collection(COLLECTION).insert_one(doc, None).await.expect("insertion failed");
            },
            _ => (),
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
