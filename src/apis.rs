pub mod v1 {
    pub mod products{
        pub mod products_routes;
        pub mod products_handler;
        mod products_model;
    }
    pub mod category{
        pub mod category_routes;
        pub mod category_handler;
        mod category_model;
    }
    pub mod v_route;
}

pub mod login {
    pub mod email;
    pub mod handler;
    pub mod model;
    pub mod response;
    pub mod login_route;
}

pub mod config;
pub mod jwt_auth;