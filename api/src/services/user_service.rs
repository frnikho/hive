use diesel::PgConnection;
use crate::dtos::user::{CreateAccessTokenRequest, CreateUserRequest, UpdateUserRequest};
use crate::entities::access_token::AccessToken;
use crate::entities::authority::Authority;
use crate::entities::pagination::Pagination;
use crate::entities::session::{TokenSession, TokenSessionKind};
use crate::entities::user::User;
use crate::exceptions::api::ApiException;
use crate::extractors::req_box::ReqBox;
use crate::repositories::access_token_repo::AccessTokenRepo;
use crate::repositories::user_repo::{UserFindOneClause, UserRepository};
use crate::templates::template::{Template, TemplateList};
use crate::utils::token::Token;

pub struct UserService;

impl UserService {
    pub fn list(db: &mut PgConnection, auth: Authority, pag: Pagination) -> Result<TemplateList<User>, ApiException> {
        Ok(TemplateList::new(Some(auth), UserRepository::list(db, &pag)?, pag))
    }

    pub fn create(db: &mut PgConnection, auth: Authority, body: CreateUserRequest) -> Result<Template<User>, ApiException> {
        let user = UserRepository::create(db, body.transform_repo(Some(auth.get_user().clone().id)))?;
        Ok(Template::new(Some(auth), Some(user)))
    }

    pub fn find(db: &mut PgConnection, auth: Authority, id: String) -> Result<Template<User>, ApiException> {
        Ok(Template::new(Some(auth), Some(UserRepository::find(db, &UserFindOneClause::Id(id))?)))
    }

    pub fn update(db: &mut PgConnection, auth: Authority, id: String, body: UpdateUserRequest) -> Result<Template<User>, ApiException> {
        let user = UserRepository::update(db, &id, body.transform_repo(Some(auth.get_user().clone().id)))?;
        Ok(Template::new(Some(auth), Some(user)))
    }

    pub fn delete(db: &mut PgConnection, auth: Authority, id: String) -> Result<Template<User>, ApiException> {
        let user = UserRepository::delete(db, &id, &Some(auth.get_user().clone().id))?;
        Ok(Template::new(Some(auth), Some(user)))
    }

    pub fn create_access_token(mut tool: ReqBox, auth: Authority, user_id: String, body: CreateAccessTokenRequest) -> Result<Template<AccessToken>, ApiException> {
        let user = UserRepository::find(&mut tool.db, &UserFindOneClause::Id(user_id))?;
        let token = AccessTokenRepo::create_access_token(&mut tool.db, &user, body.transform_repo(Token::AccessToken.generate(), None))?;
        TokenSession::new(TokenSessionKind::AccessToken(token.clone()), &user).save_to_cache(&mut tool.cache)
            .map_err(|x| x.into())?;
        Ok(Template::new(Some(auth), Some(token)))
    }

    pub fn delete_access_token(db: &mut PgConnection, auth: Authority, user_id: String, _token_id: String) -> Result<Template, ApiException> {
        let user = UserRepository::find(db, &UserFindOneClause::Id(user_id))?;
        AccessTokenRepo::revoke_access_token(db, &user, &_token_id)?;
        Ok(Template::new(Some(auth), None).with_code(200))
    }

    pub fn list_access_token(db: &mut PgConnection, auth: Authority, pag: Pagination, user_id: String) -> Result<TemplateList<AccessToken>, ApiException> {
        let user = UserRepository::find(db, &UserFindOneClause::Id(user_id))?;
        let tokens = AccessTokenRepo::list_access_token(db, &user, &pag)?;
        Ok(TemplateList::new(Some(auth), tokens, pag))
    }
}