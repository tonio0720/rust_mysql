// extern crate diesel;

use crate::utils::establish_connection;

use diesel::deserialize::QueryableByName;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel::sql_types::Text;

mod utils;

type DB = diesel::mysql::Mysql;

#[derive(Debug)]
pub struct Monster {
    monster_id: i32,
    name: String,
    type1_id: i32,
    type2_id: Option<i32>,
}

impl QueryableByName<DB> for Monster {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Monster {
            monster_id: row.get("monster_id")?,
            name: row.get("name")?,
            type1_id: row.get("type1_id")?,
            type2_id: row.get("type2_id")?,
        })
    }
}

#[derive(Debug)]
pub struct MonsterFull {
    monster_id: i32,
    name: String,
    type1_id: i32,
    type2_id: Option<i32>,
    type1_name: String,
    type2_name: Option<String>,
}

impl QueryableByName<DB> for MonsterFull {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(MonsterFull {
            monster_id: row.get("monster_id")?,
            name: row.get("name")?,
            type1_id: row.get("type1_id")?,
            type2_id: row.get("type2_id")?,
            type1_name: row.get("type1_name")?,
            type2_name: row.get("type2_name")?,
        })
    }
}

fn simple_sql() {
    let connection: MysqlConnection = establish_connection();
    let monsters: Vec<Monster> = sql_query(
        "
        SELECT
            monster_id,
            name,
            type1_id,
            type2_id
        FROM
            monster
        ",
    )
    .load(&connection)
    .unwrap();

    println!("{:?}", monsters);
}

fn prepared_statement_sql() {
    let connection: MysqlConnection = establish_connection();
    let monsters: Vec<Monster> = sql_query(
        "
        SELECT
            monster_id,
            name,
            type1_id,
            type2_id
        FROM
            monster
        WHERE
            monster_id = ?
            OR name = ?
        ",
    )
    .bind::<Integer, _>(4)
    .bind::<Text, _>("ヒトカゲ")
    .load(&connection)
    .unwrap();

    println!("{:?}", monsters[0]);
}

fn complex_sql() {
    let connection: MysqlConnection = establish_connection();
    let monsters: Vec<MonsterFull> = sql_query(
        "
        SELECT
            m.monster_id,
            m.name,
            m.type1_id,
            m.type2_id,
            t1.type_name AS type1_name,
            t2.type_name AS type2_name
        FROM
            monster m
        LEFT JOIN
            type t1
        ON
            m.type1_id = t1.type_id
        LEFT JOIN
            type t2
        ON
            m.type2_id = t2.type_id
        ",
    )
    .load(&connection)
    .unwrap();

    println!("{:?}", monsters);
}

fn main() {
    simple_sql();
    prepared_statement_sql();
    complex_sql();
}
