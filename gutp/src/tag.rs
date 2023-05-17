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

use gutp_types::GutpTag;

const GUTP_TAG_WEIGHT_DEFAULT: i32 = 0;

pub struct GutpTagModule;

impl GutpTagModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        let tag_id = params.get("id")?;

        let (sql, sql_params) = GutpTag::build_get_by_id(tag_id);
        let rowset = pg::query(&pg_addr, &sql, &sql_params)?;

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

        let sql = SqlBuilder::select_from(&GutpTag::model_name())
            .fields(&GutpTag::fields())
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let rowset = pg::query(&pg_addr, &sql, &[])?;

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

    fn list_by_subspace(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let subspace_id = params.get("subspace_id")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let sql = SqlBuilder::select_from(&GutpTag::model_name())
            .fields(&GutpTag::fields())
            .and_where_eq("subspace_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(subspace_id);
        let rowset = pg::query(&pg_addr, &sql, &[sql_param])?;

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

        let sql = SqlBuilder::select_from(&GutpTag::model_name())
            .fields(&GutpTag::fields())
            .and_where_eq("creator_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(creator_id);
        let rowset = pg::query(&pg_addr, &sql, &[sql_param])?;

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
            weight: GUTP_TAG_WEIGHT_DEFAULT,
            created_time: time,
        };

        let (sql, sql_params) = tag.build_insert();
        _ = pg::execute(&pg_addr, &sql, &sql_params)?;

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
        let (sql, sql_params) = GutpTag::build_get_by_id(id);
        let rowset = pg::query(&pg_addr, &sql, &sql_params)?;
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

                let (sql, sql_params) = tag.build_update();
                _ = pg::execute(&pg_addr, &sql, &sql_params)?;

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

        let (sql, sql_params) = GutpTag::build_delete(id);
        _ = pg::execute(&pg_addr, &sql, &sql_params)?;

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
