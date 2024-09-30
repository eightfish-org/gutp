use crate::constants::DB_URL_ENV;
use crate::utils;
use anyhow::{anyhow, bail};
use eightfish_sdk::{HandlerCRUD, Info, Module, Request, Response, Result, Router, Status};
use spin_sdk::pg::{self, ParameterValue};
use sql_builder::SqlBuilder;

use gutp_types::GutpModerator;

pub struct GutpModeratorModule;

impl GutpModeratorModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;
        let moderator_id = params.get("id").ok_or(anyhow!("id is required"))?;

        let (sql, sql_params) = GutpModerator::build_get_by_id(moderator_id);
        let rowset = pg_conn.query(&sql, &sql_params)?;

        let results = if let Some(row) = rowset.rows.into_iter().next() {
            vec![GutpModerator::from_row(row)]
        } else {
            bail!("no this item".to_string());
        };

        let info = Info {
            model_name: GutpModerator::model_name(),
            action: HandlerCRUD::GetOne,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn get_list(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpModerator::model_name())
            .fields(&GutpModerator::fields())
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let rowset = pg_conn.query(&sql, &[])?;

        let mut results: Vec<GutpModerator> = vec![];
        for row in rowset.rows {
            let sp = GutpModerator::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpModerator::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_subspace(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let subspace_id = params
            .get("subspace_id")
            .ok_or(anyhow!("subspace_id is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpModerator::model_name())
            .fields(&GutpModerator::fields())
            .and_where_eq("subspace_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(subspace_id.clone());
        let rowset = pg_conn.query(&sql, &[sql_param])?;

        let mut results: Vec<GutpModerator> = vec![];
        for row in rowset.rows {
            let sp = GutpModerator::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpModerator::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_user(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let user_id = params
            .get("user_id")
            .ok_or(anyhow!("user_id is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpModerator::model_name())
            .fields(&GutpModerator::fields())
            .and_where_eq("user_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(user_id.clone());
        let rowset = pg_conn.query(&sql, &[sql_param])?;

        let mut results: Vec<GutpModerator> = vec![];
        for row in rowset.rows {
            let sp = GutpModerator::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpModerator::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_tag(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let tag_id = params.get("tag_id").ok_or(anyhow!("tag_id is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpModerator::model_name())
            .fields(&GutpModerator::fields())
            .and_where_eq("tag_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(tag_id.clone());
        let rowset = pg_conn.query(&sql, &[sql_param])?;

        let mut results: Vec<GutpModerator> = vec![];
        for row in rowset.rows {
            let sp = GutpModerator::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpModerator::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let user_id = params
            .get("user_id")
            .ok_or(anyhow!("user_id is required"))?
            .to_owned();
        let is_subspace_moderator = params
            .get("is_smoderator")
            .ok_or(anyhow!("is_smoderator is required"))?
            .parse::<bool>()?;
        let subspace_id = params
            .get("subspace_id")
            .ok_or(anyhow!("subspace_id is required"))?
            .to_owned();
        let tag_id = params
            .get("tag_id")
            .ok_or(anyhow!("tag_id is required"))?
            .to_owned();
        let permission_level = params
            .get("permission_level")
            .ok_or(anyhow!("permission_level is required"))?
            .parse::<i16>()?;

        let id = req
            .ext()
            .get("random_str")
            .ok_or(anyhow!("generate id failed"))?
            .to_owned();
        let time = req
            .ext()
            .get("time")
            .ok_or(anyhow!("generate time failed"))?
            .parse::<i64>()?;

        let moderator = GutpModerator {
            id,
            user_id,
            is_subspace_moderator,
            subspace_id,
            tag_id,
            permission_level,
            created_time: time,
        };

        let (sql, sql_params) = moderator.build_insert();
        _ = pg_conn.execute(&sql, &sql_params)?;

        let results: Vec<GutpModerator> = vec![moderator];

        let info = Info {
            model_name: GutpModerator::model_name(),
            action: HandlerCRUD::Create,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id").ok_or(anyhow!("id is required"))?;
        let user_id = params
            .get("user_id")
            .ok_or(anyhow!("user_id is required"))?
            .to_owned();
        let is_subspace_moderator = params
            .get("is_smoderator")
            .ok_or(anyhow!("is_smoderator is required"))?
            .parse::<bool>()?;
        let subspace_id = params
            .get("subspace_id")
            .ok_or(anyhow!("subspace_id is required"))?
            .to_owned();
        let tag_id = params
            .get("tag_id")
            .ok_or(anyhow!("tag_id is required"))?
            .to_owned();
        let permission_level = params
            .get("permission_level")
            .ok_or(anyhow!("permission_level is required"))?
            .parse::<i16>()?;
        // let time = req
        //     .ext()
        //     .get("time")
        //     .ok_or(anyhow!("time is required"))?
        //     .parse::<i64>()?;

        // get the item from db, check whether obj in db
        let (sql, sql_params) = GutpModerator::build_get_by_id(id);
        let rowset = pg_conn.query(&sql, &sql_params)?;
        match rowset.rows.into_iter().next() {
            Some(row) => {
                let old_moderator = GutpModerator::from_row(row);

                let moderator = GutpModerator {
                    user_id,
                    is_subspace_moderator,
                    subspace_id,
                    tag_id,
                    permission_level,
                    ..old_moderator
                };

                let (sql, sql_params) = moderator.build_update();
                _ = pg_conn.execute(&sql, &sql_params)?;

                let results: Vec<GutpModerator> = vec![moderator];

                let info = Info {
                    model_name: GutpModerator::model_name(),
                    action: HandlerCRUD::Update,
                    extra: "".to_string(),
                };

                Ok(Response::new(Status::Successful, info, results))
            }
            None => {
                bail!("update action: no item in db");
            }
        }
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id").ok_or(anyhow!("id is required"))?;

        let (sql, sql_params) = GutpModerator::build_delete(id);
        _ = pg_conn.execute(&sql, &sql_params)?;

        let info = Info {
            model_name: GutpModerator::model_name(),
            action: HandlerCRUD::Delete,
            extra: "".to_string(),
        };
        let results: Vec<GutpModerator> = vec![];

        Ok(Response::new(Status::Successful, info, results))
    }
}

impl Module for GutpModeratorModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/gutp/v1/moderator", Self::get_one);
        router.get("/gutp/v1/moderator/list", Self::get_list);
        router.get(
            "/gutp/v1/moderator/list_by_subspace",
            Self::list_by_subspace,
        );
        router.get("/gutp/v1/moderator/list_by_user", Self::list_by_user);
        router.get("/gutp/v1/moderator/list_by_tag", Self::list_by_tag);
        router.post("/gutp/v1/moderator/create", Self::new_one);
        router.post("/gutp/v1/moderator/update", Self::update);
        router.post("/gutp/v1/moderator/delete", Self::delete);

        Ok(())
    }
}
