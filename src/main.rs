use ormlite::Connection;
use ormlite::model::*;
use uuid::Uuid;


#[derive(Model, Debug)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub age: i32,

    pub organization_id: Uuid,

    #[ormlite(many_to_one_key = organization_id)]
    pub organization: Join<Organization>,
}

#[derive(Model, Debug)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}


#[tokio::main]
async fn main() {
    // Note we don't need to insert it.
    let org = Organization {
        id: Uuid::new_v4(),
        name: "Acme".to_string(),
    };

    let user = Person {
        id: Uuid::new_v4(),
        name: "John".to_string(),
        age: 99,
        // Note we don't need to set this field. It'll be overwritten by the Join.
        organization_id: Uuid::default(),
        organization: Join::new(org),
    };

    let mut conn = ormlite::sqlite::SqliteConnection::connect(":memory:").await.unwrap();
    let user = user.insert(&mut conn).await.unwrap();
    println!("{:?}", user);
}
