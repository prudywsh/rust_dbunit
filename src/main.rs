use mysql::prelude::*;
use mysql::*;

fn get_db_conn() -> mysql::PooledConn {
    let url = "mysql://root:root@localhost:3306/spendesk_dev";
    let pool = Pool::new(url).unwrap();
    pool.get_conn().unwrap()
}

fn get_tables_names(conn: &mut mysql::PooledConn) -> Vec<String> {
    conn.query_map("show tables;", |table_name| table_name)
        .unwrap()
}

fn main() {
    let mut conn = get_db_conn();

    let tables_names = get_tables_names(&mut conn);

    println!("{:?}", tables_names);
}
