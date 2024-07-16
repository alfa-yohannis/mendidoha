use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::models::supplier::{NewSupplier, Supplier};
use crate::schema::suppliers;

pub fn create_supplier<'a>(
    conn: &mut PgConnection,
    code: &'a str,
    name: &'a str,
    created_by: Option<&'a str>,
) -> QueryResult<Supplier> {
    let random_code = crate::db::generate_code();
    let current_time = Utc::now();

    let new_supplier = NewSupplier {
        code: &random_code,
        name,
        created: current_time,
        updated: current_time,
        created_by,
        updated_by: created_by,
    };

    diesel::insert_into(suppliers::table)
        .values(&new_supplier)
        .get_result(conn)
}

pub fn get_supplier(conn: &mut PgConnection, supplier_id: i32) -> QueryResult<Supplier> {
    suppliers::table
        .find(supplier_id)
        .get_result::<Supplier>(conn)
}

pub fn update_supplier(
    conn: &mut PgConnection,
    supplier_id: i32,
    code: &str,
    name: &str,
    updated_by: Option<&str>,
) -> QueryResult<Supplier> {
    diesel::update(suppliers::table.find(supplier_id))
        .set((
            suppliers::code.eq(code),
            suppliers::name.eq(name),
            suppliers::updated.eq(Utc::now().naive_utc()),
            suppliers::updated_by.eq(updated_by),
        ))
        .get_result(conn)
}

pub fn delete_supplier(conn: &mut PgConnection, supplier_id: i32) -> QueryResult<usize> {
    diesel::delete(suppliers::table.find(supplier_id)).execute(conn)
}

pub fn list_suppliers(
    conn: &mut PgConnection,
    search: Option<String>,
) -> QueryResult<Vec<Supplier>> {
    use crate::schema::suppliers::dsl::*;

    let mut query = suppliers.into_boxed();
    if let Some(search_term) = search {
        query = query.filter(
            code.like(format!("%{}%", search_term.clone()))
                .or(name.like(format!("%{}%", search_term))),
        );
    }
    query.load::<Supplier>(conn)
}
