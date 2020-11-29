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

fn find_latest_test_database(databases: Vec<String>) -> Option<String> {
  let mut spendesk_dev_databases: Vec<String> = databases
    .into_iter()
    .filter(|database| database.starts_with("spendesk_dev_"))
    .collect();

  spendesk_dev_databases.sort();

  match spendesk_dev_databases.last() {
    Some(database) => return Some(database.clone()),
    None => return None,
  }
}

pub fn get_latest_test_database_conn() -> Option<mysql::PooledConn> {
  let url = "mysql://root:root@localhost:3306";
  let pool = Pool::new(url).unwrap();
  let mut conn = pool.get_conn().unwrap();

  let databases: Vec<String> = conn.query_map("SHOW DATABASES;", |name| name).unwrap();

  let test_database_option = find_latest_test_database(databases);

  match test_database_option {
    Some(test_database) => {
      format!("USE {};", test_database).run(&mut conn).unwrap();
      return Some(conn);
    }
    None => None,
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_find_latest_test_database_no_match() {
    let databases = vec![
      String::from("spendesk_dev"),
      String::from("localhost"),
      String::from("mysql"),
      String::from("performance_schema"),
      String::from("information_schema"),
      String::from("sys"),
    ];
    assert_eq!(super::find_latest_test_database(databases), None);
  }

  #[test]
  fn test_find_latest_test_database_one_test_database() {
    let databases = vec![
      String::from("spendesk_dev"),
      String::from("spendesk_dev_111111111"),
      String::from("localhost"),
      String::from("mysql"),
      String::from("performance_schema"),
      String::from("information_schema"),
      String::from("sys"),
    ];
    assert_eq!(
      super::find_latest_test_database(databases),
      Some(String::from("spendesk_dev_111111111"))
    );
  }

  #[test]
  fn test_find_latest_test_database_several_test_databases() {
    let databases = vec![
      String::from("spendesk_dev"),
      String::from("spendesk_dev_111111111"),
      String::from("spendesk_dev_333333333"),
      String::from("spendesk_dev_222222222"),
      String::from("localhost"),
      String::from("mysql"),
      String::from("performance_schema"),
      String::from("information_schema"),
      String::from("sys"),
    ];
    assert_eq!(
      super::find_latest_test_database(databases),
      Some(String::from("spendesk_dev_333333333"))
    );
  }
}
