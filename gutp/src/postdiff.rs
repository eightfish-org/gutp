use anyhow::bail;
use eightfish::{
    EightFishModel, HandlerCRUD, Info, Module, Request, Response, Result, Router, Status,
};
use eightfish_derive::EightFishModel;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{self, ParameterValue};
use sql_builder::SqlBuilder;

const REDIS_URL_ENV: &str = "REDIS_URL";
const DB_URL_ENV: &str = "DB_URL";
const PAGESIZE: u64 = 25;

use gutp_types::GutpPostDiff;

pub struct GutpPostDiffModule;

impl GutpPostDiffModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        let postdiff_id = params.get("id")?;

        let (sql, sql_params) = GutpPostDiff::build_get_by_id(postdiff_id);
        let rowset = pg::query(&pg_addr, &sql, &sql_params)?;

        let results = if let Some(row) = rowset.rows.next() {
            vec![GutpPostDiff::from_row(row)]
        } else {
            return bail!("no this item".to_string());
        };

        let info = Info {
            model_name: GutpPostDiff::model_name(),
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

        let sql = SqlBuilder::select_from(&GutpPostDiff::model_name())
            .fields(&GutpPostDiff::fields())
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let rowset = pg::query(&pg_addr, &sql, &[])?;

        let mut results: Vec<GutpPostDiff> = vec![];
        for row in rowset.rows {
            let sp = GutpPostDiff::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpPostDiff::model_name(),
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

        let sql = SqlBuilder::select_from(&GutpPostDiff::model_name())
            .fields(&GutpPostDiff::fields())
            .and_where_eq("post_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(post_id);
        let rowset = pg::query(&pg_addr, &sql, &[sql_param])?;

        let mut results: Vec<GutpPostDiff> = vec![];
        for row in rowset.rows {
            let sp = GutpPostDiff::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpPostDiff::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let post_id = params.get("post_id")?.to_owned();
        let diff = params.get("diff")?.to_owned();
        let version_num = params.get("version_num")?.parse::<i32>()?;

        let id = req.ext().get("random_str")?.to_owned();
        let time = req.ext().get("time")?.parse::<i64>()?;

        let postdiff = GutpPostDiff {
            id,
            post_id,
            diff,
            version_num,
            created_time: time,
        };

        let (sql, sql_params) = postdiff.build_insert();
        _ = pg::execute(&pg_addr, &sql, &sql_params)?;

        let results: Vec<GutpPostDiff> = vec![postdiff];

        let info = Info {
            model_name: GutpPostDiff::model_name(),
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
        let diff = params.get("diff")?.to_owned();
        let version_num = params.get("version_num")?.parse::<i32>()?;

        // get the item from db, check whether obj in db
        let (sql, sql_params) = GutpPostDiff::build_get_by_id(id);
        let rowset = pg::query(&pg_addr, &sql, &sql_params)?;
        match rowset.rows.next() {
            Some(row) => {
                let old_postdiff = GutpPostDiff::from_row(row);

                let postdiff = GutpPostDiff {
                    post_id,
                    diff,
                    version_num,
                    ..old_postdiff
                };

                let (sql, sql_params) = postdiff.build_update();
                _ = pg::execute(&pg_addr, &sql, &sql_params)?;

                let results: Vec<GutpPostDiff> = vec![postdiff];

                let info = Info {
                    model_name: GutpPostDiff::model_name(),
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

        let (sql, sql_params) = GutpPostDiff::build_delete(id);
        _ = pg::execute(&pg_addr, &sql, &sql_params)?;

        let info = Info {
            model_name: GutpPostDiff::model_name(),
            action: HandlerCRUD::Delete,
            extra: "".to_string(),
        };
        let results: Vec<GutpPostDiff> = vec![];

        Ok(Response::new(Status::Successful, info, results))
    }
}

impl Module for GutpPostDiffModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/v1/postdiff", Self::get_one);
        router.get("/v1/postdiff/list", Self::get_list);
        router.get("/v1/postdiff/list_by_post", Self::list_by_post);
        router.post("/v1/postdiff/create", Self::new_one);
        router.post("/v1/postdiff/update", Self::update);
        router.post("/v1/postdiff/delete", Self::delete);

        Ok(())
    }
}
