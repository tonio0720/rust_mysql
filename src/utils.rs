use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://root@localhost/pokemon";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
