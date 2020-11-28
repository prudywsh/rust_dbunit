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

fn create_new_database(database_name: &String) -> mysql::PooledConn {
  let url = "mysql://root:root@localhost:3306";
  let pool = Pool::new(url).unwrap();
  let mut conn = pool.get_conn().unwrap();

  let create_database_query = format!("CREATE DATABASE {};", database_name);
  create_database_query.run(&mut conn).unwrap();

  let use_database_query = format!("USE {};", database_name);
  use_database_query.run(&mut conn).unwrap();

  let row: Vec<String> = conn.query_map("show tables;", |name| name).unwrap();
  println!("{:?}", row);

  conn
}

fn create_tables_in_database(create_tables_query: String, conn: &mut mysql::PooledConn) {
  create_tables_query.run(conn).unwrap();
}

pub fn build_test_database(create_tables_query: String) -> (String, mysql::PooledConn) {
  let database_name = generate_database_name();
  let mut conn = create_new_database(&database_name);
  create_tables_in_database(create_tables_query, &mut conn);

  (database_name, conn)
}
