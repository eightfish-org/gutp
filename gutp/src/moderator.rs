use anyhow::bail;
use eightfish::{
    EightFishModel, HandlerCRUD, Info, Module, Request, Response, Result, Router, Status,
};
use eightfish_derive::EightFishModel;
use serde::{Deserialize, Serialize};
use spin_sdk::pg;

const REDIS_URL_ENV: &str = "REDIS_URL";
const DB_URL_ENV: &str = "DB_URL";
const PAGESIZE: u64 = 25;

use gutp_types::GutpModerator;

pub struct GutpModeratorModule;

impl GutpModeratorModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        let moderator_id = params.get("id")?;

        let (sql_statement, sql_params) = GutpModerator::build_get_one_sql_and_params(moderator_id);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let results = if let Some(row) = rowset.rows.next() {
            vec![GutpModerator::from_row(row)]
        } else {
            return bail!("no this item".to_string());
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

        let params = req.parse_urlencoded()?;

        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let (sql_statement, sql_params) =
            GutpModerator::build_get_list_sql_and_params(offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

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

        let params = req.parse_urlencoded()?;

        let subspace_id = params.get("subspace_id")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let (sql_statement, sql_params) = GutpModerator::build_get_list_by_sql_and_params(
            "subspace_id",
            subspace_id,
            offset,
            limit,
        );
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

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

        let params = req.parse_urlencoded()?;

        let user_id = params.get("user_id")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let (sql_statement, sql_params) =
            GutpModerator::build_get_list_by_sql_and_params("user_id", user_id, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

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

        let params = req.parse_urlencoded()?;

        let tag_id = params.get("tag_id")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let (sql_statement, sql_params) =
            GutpModerator::build_get_list_by_sql_and_params("tag_id", tag_id, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

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

        let params = req.parse_urlencoded()?;

        let user_id = params.get("user_id")?.to_owned();
        let is_subspace_moderator = params.get("is_smoderator")?.parse::<bool>()?;
        let subspace_id = params.get("subspace_id")?.to_owned();
        let tag_id = params.get("tag_id")?.to_owned();
        let permission_level = params.get("permission_level")?.parse::<i16>()?;

        let id = req.ext().get("random_str")?.to_owned();
        let time = req.ext().get("time")?.parse::<i64>()?;

        let moderator = GutpModerator {
            id,
            user_id,
            is_subspace_moderator,
            subspace_id,
            tag_id,
            permission_level,
            created_time: time,
        };

        let (sql_statement, sql_params) = moderator.build_insert_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

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

        let params = req.parse_urlencoded()?;

        let id = params.get("id")?;
        let user_id = params.get("user_id")?.to_owned();
        let is_subspace_moderator = params.get("is_smoderator")?.parse::<bool>()?;
        let subspace_id = params.get("subspace_id")?.to_owned();
        let tag_id = params.get("tag_id")?.to_owned();
        let permission_level = params.get("permission_level")?.parse::<i16>()?;

        // get the item from db, check whether obj in db
        let (sql_statement, sql_params) = GutpModerator::build_get_one_sql_and_params(id.as_str());
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        match rowset.rows.next() {
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

                let (sql_statement, sql_params) = moderator.build_update_sql_and_params();
                let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

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

        let params = req.parse_urlencoded()?;

        let id = params.get("id")?;

        let (sql_statement, sql_params) = GutpModerator::build_delete_sql_and_params(id.as_str());
        let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

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
        router.get("/v1/moderator", Self::get_one);
        router.get("/v1/moderator/list", Self::get_list);
        router.get("/v1/moderator/list_by_subspace", Self::list_by_subspace);
        router.get("/v1/moderator/list_by_user", Self::list_by_user);
        router.get("/v1/moderator/list_by_tag", Self::list_by_tag);
        router.post("/v1/moderator/create", Self::new_one);
        router.post("/v1/moderator/update", Self::update);
        router.post("/v1/moderator/delete", Self::delete);

        Ok(())
    }
}
