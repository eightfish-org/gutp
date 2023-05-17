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

use gutp_types::GutpTag;

const GutpTagWeightDefault: i32 = 0;

pub struct GutpTagModule;

impl GutpTagModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        let tag_id = params.get("id")?;

        let (sql_statement, sql_params) = GutpTag::build_get_one_sql_and_params(tag_id);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let results = if let Some(row) = rowset.rows.next() {
            vec![GutpTag::from_row(row)]
        } else {
            return bail!("no this item".to_string());
        };

        let info = Info {
            model_name: GutpTag::model_name(),
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

        let (sql_statement, sql_params) = GutpTag::build_get_list_sql_and_params(offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let mut results: Vec<GutpTag> = vec![];
        for row in rowset.rows {
            let sp = GutpTag::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpTag::model_name(),
            action: HandlerCRUD::GetOne,
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

        let (sql_statement, sql_params) =
            GutpTag::build_get_list_by_sql_and_params("subspace_id", subspace_id, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let mut results: Vec<GutpTag> = vec![];
        for row in rowset.rows {
            let sp = GutpTag::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpTag::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_creator(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let creator_id = params.get("creator_id")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let (sql_statement, sql_params) =
            GutpTag::build_get_list_by_sql_and_params("creator_id", creator_id, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let mut results: Vec<GutpTag> = vec![];
        for row in rowset.rows {
            let sp = GutpTag::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpTag::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let caption = params.get("caption")?.to_owned();
        let subspace_id = params.get("subspace_id")?.to_owned();
        let creator_id = params.get("creator_id")?.to_owned();
        let is_public = params.get("is_public")?.parse::<bool>()?;

        let id = req.ext().get("random_str")?.to_owned();
        let time = req.ext().get("time")?.parse::<i64>()?;

        let tag = GutpTag {
            id,
            caption,
            subspace_id,
            creator_id,
            is_public,
            weight: GutpTagWeightDefault,
            created_time: time,
        };

        // construct a sql statement and param
        let (sql_statement, sql_params) = tag.build_insert_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

        let results: Vec<GutpTag> = vec![tag];

        let info = Info {
            model_name: GutpTag::model_name(),
            action: HandlerCRUD::Create,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id")?;
        let caption = params.get("caption")?.to_owned();
        let subspace_id = params.get("subspace_id")?.to_owned();
        let creator_id = params.get("creator_id")?.to_owned();
        let is_public = params.get("is_public")?.parse::<bool>()?;

        // get the item from db, check whether obj in db
        let (sql_statement, sql_params) = GutpTag::build_get_one_sql_and_params(id.as_str());
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        match rowset.rows.next() {
            Some(row) => {
                let old_tag = GutpTag::from_row(row);

                let tag = GutpTag {
                    caption,
                    subspace_id,
                    creator_id,
                    is_public,
                    ..old_tag
                };

                let (sql_statement, sql_params) = tag.build_update_sql_and_params();
                let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

                let results: Vec<GutpTag> = vec![tag];

                let info = Info {
                    model_name: GutpTag::model_name(),
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

        let (sql_statement, sql_params) = GutpTag::build_delete_sql_and_params(id.as_str());
        let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

        let info = Info {
            model_name: GutpTag::model_name(),
            action: HandlerCRUD::Delete,
            extra: "".to_string(),
        };
        let results: Vec<GutpTag> = vec![];

        Ok(Response::new(Status::Successful, info, results))
    }
}

impl Module for GutpTagModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/v1/tag", Self::get_one);
        router.get("/v1/tag/list", Self::get_list);
        router.get("/v1/tag/list_by_subspace", Self::list_by_subspace);
        router.get("/v1/tag/list_by_creator", Self::list_by_creator);
        router.post("/v1/tag/create", Self::new_one);
        router.post("/v1/tag/update", Self::update);
        router.post("/v1/tag/delete", Self::delete);

        Ok(())
    }
}
