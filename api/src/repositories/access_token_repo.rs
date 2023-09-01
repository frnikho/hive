use chrono::NaiveDateTime;
use diesel::{Connection, PgConnection};
use crate::entities::access_token::AccessToken;
use crate::entities::pagination::Pagination;
use crate::entities::user::User;
use crate::exceptions::api::ApiException;
use crate::exceptions::db::DatabaseException;
use crate::models::access_token::{AccessTokenModel, CreateAccessTokenModel};

pub struct AccessTokenRepo;

pub struct CreateAccessToken {
    pub name: String,
    pub key: String,
    pub expiration: Option<NaiveDateTime>,
    pub created_by_user_id: Option<String>,
}

impl CreateAccessToken {
    pub fn into_model(self, user: &User) -> CreateAccessTokenModel {
        CreateAccessTokenModel {
            name: self.name,
            key: self.key,
            expiration: self.expiration,
            created_by_user_id: user.id.clone(),
        }
    }
}

impl AccessTokenRepo {

    pub fn list_access_token(conn: &mut PgConnection, user: &User, pag: &Pagination) -> Result<Vec<AccessToken>, ApiException> {
        user.clone().into_model().list_access_token(conn, pag)
            .map(|x| AccessToken::from_vec_model(x))
            .map_err(|x| x.into())
    }

    pub fn create_access_token(conn: &mut PgConnection, user: &User, body: CreateAccessToken) -> Result<AccessToken, ApiException> {
        conn.transaction::<AccessToken, DatabaseException, _>(|x| {
            let token = AccessTokenModel::create(x, body.into_model(user)).map(|x| AccessToken::from(x))?;
            let user = user.clone().into_model();
            user.add_access_token(x, &token.id)?;
            Ok(token)
        }).map_err(|x| x.into())
    }

    pub fn revoke_access_token(conn: &mut PgConnection, user: &User, id: &str) -> Result<(), ApiException> {
        conn.transaction::<(), DatabaseException, _>(|x| {
            user.clone().into_model().remove_access_token(x, &id.to_string())
        }).map_err(|x| x.into())
    }

}