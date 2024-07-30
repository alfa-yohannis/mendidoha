// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        #[max_length = 10]
        code -> Varchar,
        name -> Varchar,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    goods (id) {
        id -> Int4,
        #[max_length = 10]
        code -> Varchar,
        name -> Varchar,
        price -> Numeric,
        quantity -> Numeric,
        #[max_length = 50]
        unit -> Nullable<Varchar>,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    purchase_order_details (id) {
        id -> Int4,
        order_id -> Int4,
        item_id -> Int4,
        item_type -> Varchar,
        quantity -> Numeric,
        #[max_length = 50]
        unit -> Nullable<Varchar>,
        unit_price -> Numeric,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    purchase_orders (id) {
        id -> Int4,
        #[max_length = 10]
        code -> Varchar,
        order_date -> Date,
        supplier_id -> Int4,
        total_amount -> Numeric,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    sales_order_details (id) {
        id -> Int4,
        order_id -> Int4,
        item_id -> Int4,
        item_type -> Varchar,
        quantity -> Numeric,
        #[max_length = 50]
        unit -> Nullable<Varchar>,
        unit_price -> Numeric,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    sales_orders (id) {
        id -> Int4,
        #[max_length = 10]
        code -> Varchar,
        order_date -> Date,
        customer_id -> Int4,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    services (id) {
        id -> Int4,
        #[max_length = 10]
        code -> Varchar,
        name -> Varchar,
        price -> Numeric,
        #[max_length = 50]
        unit -> Nullable<Varchar>,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        #[max_length = 10]
        user_code -> Varchar,
        #[max_length = 36]
        device_id -> Varchar,
        #[max_length = 36]
        session_id -> Varchar,
        start_time -> Timestamptz,
        expiry_time -> Timestamptz,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    suppliers (id) {
        id -> Int4,
        #[max_length = 10]
        code -> Varchar,
        name -> Varchar,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 10]
        code -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 32]
        password -> Varchar,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        middle_name -> Nullable<Varchar>,
        #[max_length = 50]
        last_name -> Varchar,
        created -> Timestamptz,
        updated -> Timestamptz,
        #[max_length = 10]
        created_by -> Nullable<Varchar>,
        #[max_length = 10]
        updated_by -> Nullable<Varchar>,
    }
}

diesel::joinable!(purchase_order_details -> purchase_orders (order_id));
diesel::joinable!(purchase_orders -> suppliers (supplier_id));
diesel::joinable!(sales_order_details -> sales_orders (order_id));
diesel::joinable!(sales_orders -> customers (customer_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    goods,
    purchase_order_details,
    purchase_orders,
    sales_order_details,
    sales_orders,
    services,
    sessions,
    suppliers,
    users,
);
