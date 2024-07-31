use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;
use log::info;

use crate::models::good::{Goods, NewGoods};
use crate::schema::goods;


pub fn create_goods<'a>(
    conn: &mut PgConnection,
    code: &'a str,
    name: &'a str,
    currency: &'a str,
    price: f32,
    quantity: f32,
    unit: Option<&'a str>,
    created_by: Option<&'a str>,
) -> QueryResult<Goods> {
    let random_code = crate::db::generate_code(); // Assuming this function generates a unique code
    let current_time = Utc::now();

    let new_goods = NewGoods {
        code: &random_code,
        name,
        currency,
        price,
        quantity,
        unit,
        created: current_time,
        updated: current_time,
        created_by,
        updated_by: created_by,
    };

    diesel::insert_into(goods::table)
        .values(&new_goods)
        .get_result(conn)
}

pub fn get_goods(conn: &mut PgConnection, goods_code: &str) -> QueryResult<Goods> {
    goods::table
        .filter(goods::code.eq(goods_code))
        .get_result::<Goods>(conn)
}

pub fn update_goods(
    conn: &mut PgConnection,
    code: &str,
    name: &str,
    currency: &str,
    price: f32,
    quantity: f32,
    unit: Option<&str>,
    updated_by: Option<&str>,
) -> QueryResult<Goods> {
    diesel::update(goods::table.filter(goods::code.eq(code)))
        .set((
            goods::name.eq(name),
            goods::currency.eq(currency),
            goods::price.eq(price),
            goods::quantity.eq(quantity),
            goods::unit.eq(unit),
            goods::updated.eq(Utc::now()),
            goods::updated_by.eq(updated_by),
        ))
        .get_result(conn)
}

pub fn delete_goods(conn: &mut PgConnection, goods_code: &str) -> QueryResult<usize> {
    diesel::delete(goods::table.filter(goods::code.eq(goods_code))).execute(conn)
}

pub fn list_goods(conn: &mut PgConnection, search_term: &str) -> QueryResult<Vec<Goods>> {
    use crate::schema::goods::dsl::*;

    let mut query = goods.into_boxed();

    let search_pattern = format!("%{}%", search_term);
    info!("Search term: {}", search_term); // Log the search term
    info!("Search pattern: {}", search_pattern); // Log the search pattern
    query = query.filter(
        name.like(search_pattern.clone())
            .or(code.like(search_pattern)),
    );

    let q = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    info!("SQL Query: {:?}", q);
    query.load::<Goods>(conn)
}
