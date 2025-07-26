use crate::constants::{DB_URL, REDIS_URL};
use crate::utils;
use anyhow::{anyhow, bail};
use eightfish_sdk::{EightFishModel, Module, Request, Response, Result, Router, StatusCode};
use gutp_types::{GutpUser, GutpUserRole, GutpUserStatus};
use serde_json::json;
use spin_sdk::pg::ParameterValue;
use spin_worker::{
    sql_create_one, sql_delete, sql_delete_one, sql_query, sql_query_one, sql_update,
    sql_update_one,
};
use sql_builder::SqlBuilder;

pub struct GutpUserModule;

impl GutpUserModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let entity_id = params.get("id").ok_or(anyhow!("id is required"))?;

        let user = sql_query_one!(GutpUser, &entity_id);
        match user {
            Some(user) => {
                let results: Vec<GutpUser> = vec![user];

                Ok(Response::new_check(results))
            }
            None => {
                bail!("get user: no user in db");
            }
        }
    }

    fn get_by_account(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let account = params
            .get("account")
            .ok_or(anyhow!("account is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpUser::model_name())
            .fields(&GutpUser::fields())
            .and_where_eq("account", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_params = vec![ParameterValue::Str(account.clone())];
        let users = sql_query!(GutpUser, &sql, &sql_params);

        Ok(Response::new_check(users))
    }

    fn new_user(req: &mut Request) -> Result<Response> {
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

        let user = GutpUser {
            id,
            account,
            oauth_source,
            nickname,
            avatar,
            role: GutpUserRole::Normal as i16,
            status: GutpUserStatus::Normal as i16,
            created_time: time,
            data_source: "".to_string(),
        };

        match sql_create_one!(req, user) {
            Ok(user) => {
                let results: Vec<GutpUser> = vec![user];

                Ok(Response::new_check(results))
            }
            Err(_) => {
                let json_result = json!({
                    "status": "failed",
                    "info": "Error when creating a new user",
                });
                Ok(Response::new_uncheck(StatusCode::BAD_REQUEST, json_result))
            }
        }
    }

    fn update(req: &mut Request) -> Result<Response> {
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

        let user = sql_query_one!(GutpUser, &id);
        match user {
            Some(old_user) => {
                let new_user: GutpUser = GutpUser {
                    account,
                    oauth_source,
                    nickname,
                    avatar,
                    ..old_user
                };

                match sql_update_one!(req, new_user) {
                    Ok(user) => Ok(Response::new_check(vec![user])),
                    Err(_) => {
                        bail!("update user info error: db operation error")
                    }
                }
            }
            None => {
                bail!("update user info error: no user in db");
            }
        }
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;
        let id = params.get("id").ok_or(anyhow!("id is required"))?;

        let user = sql_query_one!(GutpUser, &id);
        match user {
            Some(user) => match sql_delete_one!(req, user) {
                Ok(user) => Ok(Response::new_check(vec![user])),
                Err(_) => {
                    bail!("delete user error: db operation error")
                }
            },
            None => {
                bail!("delete user error: no user in db");
            }
        }
    }
}

impl Module for GutpUserModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/gutp/v1/user", Self::get_one);
        router.get("/gutp/v1/user/get_by_account", Self::get_by_account);
        router.post("/gutp/v1/user/create", Self::new_user);
        router.put("/gutp/v1/user/update", Self::update);
        router.delete("/gutp/v1/user/delete", Self::delete);

        Ok(())
    }
}
