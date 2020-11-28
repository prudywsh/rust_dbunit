mod create_table_query_cleanup;

use crate::create_table_query_cleanup::*;
use mysql::prelude::*;
use mysql::*;
use std::time::SystemTime;

fn generate_database_name() -> String {
    let now_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    format!("spendesk_dev_{}", now_timestamp)
}

fn create_new_database() -> mysql::PooledConn {
    let url = "mysql://root:root@localhost:3306";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let database_name = generate_database_name();

    let create_database_query = format!("CREATE DATABASE {};", database_name);
    create_database_query.run(&mut conn).unwrap();

    let use_database_query = format!("USE {};", database_name);
    use_database_query.run(&mut conn).unwrap();

    let row: Vec<String> = conn.query_map("show tables;", |name| name).unwrap();
    println!("{:?}", row);

    conn
}

fn get_db_conn() -> mysql::PooledConn {
    let url = "mysql://root:root@localhost:3306/spendesk_dev";
    let pool = Pool::new(url).unwrap();
    pool.get_conn().unwrap()
}

fn get_tables_names(conn: &mut mysql::PooledConn) -> Vec<String> {
    // conn.query_map("SHOW TABLES;", |table_name| table_name)
    //    .unwrap()
    vec![
        String::from("companies"),
        String::from("users"),
        String::from("accounts"),
    ]
}

fn get_table_create_query(conn: &mut mysql::PooledConn, table_name: String) -> String {
    let query = format!("SHOW CREATE TABLE {}", table_name);
    let row = conn.query_first(query).unwrap();
    let (_, create_table_query): (String, String) = row.unwrap();
    create_table_query
}

fn build_cleaned_create_tables_query(conn: &mut mysql::PooledConn) -> String {
    let tables_names = get_tables_names(conn);
    let mut cleaned_create_table_queries: Vec<String> = vec![];

    for table_name in tables_names {
        let create_table_query = get_table_create_query(conn, table_name);
        let cleaned_create_table_query = remove_foreign_keys_constraints(&create_table_query);
        cleaned_create_table_queries.push(cleaned_create_table_query);
    }

    cleaned_create_table_queries.join("\n")
}

fn main() {
    let mut conn = get_db_conn();
    // let query = build_cleaned_create_tables_query(&mut conn);

    // println!("{:}", query);
    create_new_database();
}
