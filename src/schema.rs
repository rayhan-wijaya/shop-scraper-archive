// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Mediumint"))]
    pub struct ProductsIdMediumint;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProductsIdMediumint;

    products (id) {
        id -> ProductsIdMediumint,
        #[max_length = 50]
        name -> Varchar,
        stars -> Nullable<Decimal>,
        #[max_length = 255]
        url -> Varchar,
        price_in_idr -> Integer,
    }
}
