mod create_table_query_cleanup;

use crate::create_table_query_cleanup::*;
use mysql::prelude::*;
use mysql::*;

fn get_db_conn() -> mysql::PooledConn {
    let url = "mysql://root:root@localhost:3306/spendesk_dev";
    let pool = Pool::new(url).unwrap();
    pool.get_conn().unwrap()
}

fn get_tables_names(conn: &mut mysql::PooledConn) -> Vec<String> {
    conn.query_map("SHOW TABLES;", |table_name| table_name)
        .unwrap()
}

fn get_table_create_query(conn: &mut mysql::PooledConn, table_name: String) -> String {
    let query = format!("SHOW CREATE TABLE {}", table_name);
    let row = conn.query_first(query).unwrap();
    let (_, create_table_query): (String, String) = row.unwrap();
    create_table_query
}

fn main() {
    let mut conn = get_db_conn();

    // let tables_names = get_tables_names(&mut conn);
    // println!("{:?}", tables_names);

    let create_table_query = get_table_create_query(&mut conn, String::from("budgets"));
    println!("{:}", create_table_query);
}
