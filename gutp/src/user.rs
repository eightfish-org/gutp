use crate::constants::DB_URL_ENV;
use crate::utils;
use anyhow::{anyhow, bail};
use eightfish::{HandlerCRUD, Info, Module, Request, Response, Result, Router, Status};
use gutp_types::GutpUser;
use spin_sdk::pg::{self, ParameterValue};
use sql_builder::SqlBuilder;

enum GutpUserStatus {
    Normal = 0,
    Frozen = 1,
    Forbidden = 2,
    Deleted = 3,
}

enum GutpUserRole {
    Normal = 0,
}

pub struct GutpUserModule;

impl GutpUserModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let entity_id = params.get("id").ok_or(anyhow!("id is required"))?;

        let (sql, sql_params) = GutpUser::build_get_by_id(entity_id);
        let rowset = pg::query(&pg_addr, &sql, &sql_params)?;

        let mut results: Vec<GutpUser> = vec![];
        for row in rowset.rows {
            let article = GutpUser::from_row(row);
            results.push(article);
        }

        let info = Info {
            model_name: GutpUser::model_name(),
            action: HandlerCRUD::GetOne,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn get_by_account(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let account = params
            .get("account")
            .ok_or(anyhow!("account is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpUser::model_name())
            .fields(&GutpUser::fields())
            .and_where_eq("account", "$1")
            .order_desc("signup_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(&account);
        let rowset = pg::query(&pg_addr, &sql, &[sql_param])?;

        let mut results: Vec<GutpUser> = vec![];
        for row in rowset.rows {
            let article = GutpUser::from_row(row);
            results.push(article);
        }

        let info = Info {
            model_name: GutpUser::model_name(),
            action: HandlerCRUD::GetOne,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn new_user(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let account = params
            .get("account")
            .ok_or(anyhow!("account is required"))?
            .to_owned();
        let oauth_source = params
            .get("oauth_source")
            .ok_or(anyhow!("oauth_source is required"))?
            .to_owned();
        let nickname = params
            .get("nickname")
            .ok_or(anyhow!("nickname is required"))?
            .to_owned();
        let avatar = params
            .get("avatar")
            .ok_or(anyhow!("avatar is required"))?
            .to_owned();

        let id = req
            .ext()
            .get("random_str")
            .ok_or(anyhow!("random_str is required"))?
            .to_owned();
        let time = req
            .ext()
            .get("time")
            .ok_or(anyhow!("time is required"))?
            .parse::<i64>()?;

        let article = GutpUser {
            id,
            account,
            oauth_source,
            nickname,
            avatar,
            role: GutpUserRole::Normal as i16,
            status: GutpUserStatus::Normal as i16,
            created_time: time,
        };

        let (sql, sql_params) = article.build_insert();
        _ = pg::execute(&pg_addr, &sql, &sql_params);

        let results: Vec<GutpUser> = vec![article];

        let info = Info {
            model_name: GutpUser::model_name(),
            action: HandlerCRUD::Create,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded()?;

        let id = params.get("id").ok_or(anyhow!("id is required"))?;
        let account = params
            .get("account")
            .ok_or(anyhow!("account is required"))?
            .to_owned();
        let oauth_source = params
            .get("oauth_source")
            .ok_or(anyhow!("oauth_source is required"))?
            .to_owned();
        let nickname = params
            .get("nickname")
            .ok_or(anyhow!("nickname is required"))?
            .to_owned();
        let avatar = params
            .get("avatar")
            .ok_or(anyhow!("avatar is required"))?
            .to_owned();

        // get the item from db, check whether obj in db
        let (sql, sql_params) = GutpUser::build_get_by_id(id);
        let rowset = pg::query(&pg_addr, &sql, &sql_params)?;
        match rowset.rows.into_iter().next() {
            Some(row) => {
                let old_user = GutpUser::from_row(row);

                let user: GutpUser = GutpUser {
                    account,
                    oauth_source,
                    nickname,
                    avatar,
                    ..old_user
                };

                let (sql, sql_params) = user.build_update();
                _ = pg::execute(&pg_addr, &sql, &sql_params)?;

                let results: Vec<GutpUser> = vec![user];

                let info = Info {
                    model_name: GutpUser::model_name(),
                    action: HandlerCRUD::Update,
                    extra: "".to_string(),
                };

                Ok(Response::new(Status::Successful, info, results))
            }
            None => {
                bail!("update action: no item in db")
            }
        }
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id").ok_or(anyhow!("id is required"))?;

        let (sql, sql_params) = GutpUser::build_delete(id);
        _ = pg::execute(&pg_addr, &sql, &sql_params);

        let results: Vec<GutpUser> = vec![];

        let info = Info {
            model_name: GutpUser::model_name(),
            action: HandlerCRUD::Delete,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }
}

impl Module for GutpUserModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/v1/user", Self::get_one);
        router.get("/v1/user/get_by_account", Self::get_by_account);
        router.post("/v1/user/create", Self::new_user);
        router.post("/v1/user/update", Self::update);
        router.post("/v1/user/delete", Self::delete);

        Ok(())
    }
}
