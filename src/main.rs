mod create_table_query_cleanup;
mod source_database_schema;
mod test_database;

use mysql::prelude::*;
use mysql::*;

use crate::source_database_schema::*;
use crate::test_database::*;

fn main() {
    let create_tables_query = get_source_database_cleaned_create_tables_query();
    println!("{:}", create_tables_query);

    let (database_name, test_database_conn) = build_test_database(create_tables_query);
    println!("{:}", database_name);

    let mut conn = get_latest_test_database_conn().unwrap();
    let v: Vec<String> = conn
        .query_map("SHOW TABLES;", |table_name| table_name)
        .unwrap();
    println!("{:?}", v);
}
