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

use gutp_types::GutpPostTag;

pub struct GutpPostTagModule;

impl GutpPostTagModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        let posttag_id = params.get("id")?;

        let (sql_statement, sql_params) = GutpPostTag::build_get_one_sql_and_params(posttag_id);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let results = if let Some(row) = rowset.rows.next() {
            vec![GutpPostTag::from_row(row)]
        } else {
            return bail!("no this item".to_string());
        };

        let info = Info {
            model_name: GutpPostTag::model_name(),
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

        let (sql_statement, sql_params) = GutpPostTag::build_get_list_sql_and_params(offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let mut results: Vec<GutpPostTag> = vec![];
        for row in rowset.rows {
            let sp = GutpPostTag::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpPostTag::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_post(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let post_id = params.get("post_id")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let (sql_statement, sql_params) =
            GutpPostTag::build_get_list_by_sql_and_params("post_id", post_id, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let mut results: Vec<GutpPostTag> = vec![];
        for row in rowset.rows {
            let sp = GutpPostTag::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpPostTag::model_name(),
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
            GutpPostTag::build_get_list_by_sql_and_params("tag_id", tag_id, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let mut results: Vec<GutpPostTag> = vec![];
        for row in rowset.rows {
            let sp = GutpPostTag::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpPostTag::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let post_id = params.get("post_id")?.to_owned();
        let tag_id = params.get("tag_id")?.to_owned();

        let id = req.ext().get("random_str")?.to_owned();
        let time = req.ext().get("time")?.parse::<i64>()?;

        let posttag = GutpPostTag {
            id,
            post_id,
            tag_id,
            created_time: time,
        };

        // construct a sql statement and param
        let (sql_statement, sql_params) = posttag.build_insert_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

        let results: Vec<GutpPostTag> = vec![posttag];

        let info = Info {
            model_name: GutpPostTag::model_name(),
            action: HandlerCRUD::Create,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id")?;
        let post_id = params.get("post_id")?.to_owned();
        let tag_id = params.get("tag_id")?.to_owned();

        // get the item from db, check whether obj in db
        let (sql_statement, sql_params) = GutpPostTag::build_get_one_sql_and_params(id.as_str());
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        match rowset.rows.next() {
            Some(row) => {
                let old_posttag = GutpPostTag::from_row(row);

                let posttag = GutpPostTag {
                    post_id,
                    tag_id,
                    ..old_posttag
                };

                let (sql_statement, sql_params) = posttag.build_update_sql_and_params();
                let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

                let results: Vec<GutpPostTag> = vec![posttag];

                let info = Info {
                    model_name: GutpPostTag::model_name(),
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

        let (sql_statement, sql_params) = GutpPostTag::build_delete_sql_and_params(id.as_str());
        let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

        let info = Info {
            model_name: GutpPostTag::model_name(),
            action: HandlerCRUD::Delete,
            extra: "".to_string(),
        };
        let results: Vec<GutpPostTag> = vec![];

        Ok(Response::new(Status::Successful, info, results))
    }
}

impl Module for GutpPostTagModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/v1/posttag", Self::get_one);
        router.get("/v1/posttag/list", Self::get_list);
        router.get("/v1/posttag/list_by_post", Self::list_by_post);
        router.get("/v1/posttag/list_by_tag", Self::list_by_tag);
        router.post("/v1/posttag/create", Self::new_one);
        router.post("/v1/posttag/update", Self::update);
        router.post("/v1/posttag/delete", Self::delete);

        Ok(())
    }
}
