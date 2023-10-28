use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::Thing;
use surrealdb::{Error, Surreal};
use surrealdb_extra::query::statement::StatementBuilder;
use surrealdb_extra::table::Table;

#[allow(dead_code)]
#[derive(Debug, Default, Table, Serialize, Deserialize, Clone, PartialEq)]
#[table(name = "test_test")]
pub struct Test {
    id: Option<Thing>,
    name: String,
    n: Option<usize>,
}

async fn database() -> Surreal<Db> {
    let db = Surreal::new::<Mem>(()).await.unwrap();

    db.use_ns("ns").use_db("db").await.unwrap();

    db
}

#[test]
fn table_derive_init() {
    assert_eq!("test_test", Test::TABLE_NAME)
}

#[test]
fn table_derive_get_id() {
    let t = Test {
        id: Some(Thing::from(("test", "test"))),
        name: "".to_string(),
        ..Test::default()
    };
    assert_eq!(t.get_id().clone().unwrap(), Thing::from(("test", "test")))
}

#[test]
fn table_derive_set_id() {
    let mut t = Test {
        name: "".to_string(),
        ..Test::default()
    };

    t.set_id("test");

    assert_eq!(t.get_id().clone().unwrap(), Thing::from(("test_test", "test")))
}

#[tokio::test]
async fn table_create() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test".to_string(),
        ..Test::default()
    };

    let tc = t.clone().create(&db).await.unwrap();

    assert_eq!(t.name, tc.first().unwrap().name);
}

#[tokio::test]
async fn table_db_get_by_id() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
        ..Test::default()
    };

    let tc = t.create(&db).await.unwrap();

    let tc = tc.first().unwrap();
    let tc_id = tc.clone().id.unwrap();

    let op_t = Test::get_by_id(tc_id.id.to_raw(), &db).await.unwrap();

    assert!(op_t.is_some());
    assert_eq!(op_t.unwrap().get_id().clone().unwrap(), tc_id)
}

#[tokio::test]
async fn table_delete() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
        ..Test::default()
    };

    let tc = t.create(&db).await.unwrap();

    let tc_id = tc.first().unwrap().clone().id;

    assert!(tc_id.is_some());

    let td = Test::delete(tc_id.unwrap().id.to_raw(), &db).await.unwrap();

    let op_td = Test::get_by_id(td.unwrap().get_id().clone().unwrap().id.to_raw(), &db).await.unwrap();

    assert!(op_td.is_none())
}

#[tokio::test]
async fn table_get_all() {
    let db = database().await;

    for _ in 0..10 {
        let t = Test {
            id: None,
            name: "test data".to_string(),
            ..Test::default()
        };

        let _ = t.create(&db).await.unwrap();
    }

    let vt = Test::get_all(&db).await.unwrap();

    assert_eq!(vt.len(), 10);
}

#[tokio::test]
async fn table_update() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
        ..Test::default()
    };

    let tc = t.create(&db).await.unwrap();

    let mut tc = tc.first().unwrap().clone();

    tc.name = "test".to_string();

    let tu = tc.clone().update(&db).await.unwrap();

    assert!(tu.is_some());
    assert_eq!(tu.unwrap().name, tc.name);
}

#[tokio::test]
async fn select_field() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
        ..Test::default()
    };

    let tc = t.create(&db).await.unwrap();
    let tc = tc.first().unwrap().clone();

    let mut q = db.select_builder().what(Test::TABLE_NAME).field("name").to_query().await.unwrap();

    let res: Vec<Test> = q.take(0).unwrap();

    let test_res = res.first().unwrap().clone();

    assert_eq!(tc.name, test_res.name);

    assert!(test_res.id.is_none());

    assert_ne!(tc.id, test_res.id);
}

#[tokio::test]
async fn select_id_name_not_selected_error() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
        ..Test::default()
    };

    let _tc = t.create(&db).await.unwrap();

    let mut q = db.select_builder().what(Test::TABLE_NAME).field("id").to_query().await.unwrap();

    let res: Result<Vec<Test>, Error> = q.take(0);

    assert!(res.is_err());
}

#[tokio::test]
async fn select_id_name_selected_success() {
    let db = database().await;

    let t = Test {
        id: None,
        name: "test data".to_string(),
        ..Test::default()
    };

    let tc = t.create(&db).await.unwrap();
    let tc = tc.first().unwrap().clone();

    let mut q = db.select_builder().what(Test::TABLE_NAME).field("id").field("name").to_query().await.unwrap();

    let res: Vec<Test> = q.take(0).unwrap();

    let test_res = res.first().unwrap().clone();

    assert_eq!(tc.name, test_res.name);

    assert!(test_res.id.is_some());

    assert_eq!(tc.id, test_res.id);
}
