use std::net::SocketAddr;

use super::logging;
use crate::{
    database,
    models::user::{UserIN, UserORM, UserOUT},
};

use axum::{
    extract::{ConnectInfo, Json},
    response::IntoResponse,
};

use rusqlite::Connection;

pub struct UserRoutes {
    conn: Connection,
}

impl UserRoutes {
    fn new() -> Self {
        Self {
            conn: database::connect(),
        }
    }

    pub async fn create(
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        Json(user): Json<UserIN>,
    ) -> impl IntoResponse {
        logging(&format!("POST from {addr} /user"));
        // Create a new user
        let orm = UserORM::new(Self::new().conn);
        let id = orm.create(&user);

        Json(UserOUT {
            id,
            firstname: user.firstname.clone(),
            lastname: user.lastname.clone(),
            email: user.email.clone(),
            phone_1: user.phone_1.clone(),
            phone_2: user.phone_2.clone(),
            address: user.address.clone(),
            city: user.city.clone(),
            country: user.country.clone(),
            biography: user.biography.clone(),
            picture: user.picture.clone(),
            wallpaper: user.wallpaper.clone(),
        })
    }
}
