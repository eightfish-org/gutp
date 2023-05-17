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

use gutp_types::GutpSubspace;

enum GutpSubspaceStatus {
    Normal = 0,
    Frozen = 1,
    Forbidden = 2,
    Deleted = 3,
}

enum GutpSubspaceWeight {
    Normal = 0,
    Low = -1,
    VeryLow = -2,
    SuperLow = -3,
    High = 1,
    VeryHigh = 2,
    SuperHigh = 2,
}

pub struct GutpSubspaceModule;

impl GutpSubspaceModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let subspace_id = params.get("id")?;

        let (sql, sql_params) = GutpSubspace::build_get_by_id(subspace_id);
        let rowset = pg::query(&pg_addr, &sql, &sql_params)?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }
        // println!("in handler subspace get_one: results: {:?}", results);

        let info = Info {
            model_name: GutpSubspace::model_name(),
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

        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let rowset = pg::query(&pg_addr, &sql, &[])?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_owner(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        // println!("in handler subspace get_one: params: {:?}", params);

        let owner_id = params.get("owner_id")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .and_where_eq("owner_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(owner_id);
        let rowset = pg::query(&pg_addr, &sql, &[sql_param])?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_profession(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        // println!("in handler subspace get_one: params: {:?}", params);

        let profession = params.get("profession")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .and_where_eq("profession", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(profession);
        let rowset = pg::query(&pg_addr, &sql, &[sql_param])?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_appid(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let appid = params.get("appid")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .and_where_eq("appid", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(appid);
        let rowset = pg::query(&pg_addr, &sql, &[sql_param])?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let title = params.get("title")?.to_owned();
        let description = params.get("description")?.to_owned();
        let banner = params.get("banner")?.to_owned();
        let owner_id = params.get("owner_id")?.to_owned();
        let profession = params.get("profession")?.to_owned();
        let appid = params.get("appid")?.to_owned();
        let is_public = params.get("is_public")?.parse::<bool>()?;

        let id = req.ext().get("random_str")?.to_owned();
        let time = req.ext().get("time")?.parse::<i64>()?;

        let subspace = GutpSubspace {
            id,
            title,
            description,
            banner,
            owner_id,
            profession,
            appid,
            is_public,
            status: GutpSubspaceStatus::Normal,
            weight: GutpSubspaceWeight::Normal,
            created_time: time,
        };

        let (sql, sql_params) = subspace.build_insert();
        _ = pg::execute(&pg_addr, &sql, &sql_params)?;

        let results: Vec<GutpSubspace> = vec![subspace];

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: HandlerCRUD::Create,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id")?;
        let title = params.get("title")?.to_owned();
        let description = params.get("description")?.to_owned();
        let banner = params.get("banner")?.to_owned();
        let owner_id = params.get("owner_id")?.to_owned();
        let profession = params.get("profession")?.to_owned();
        let appid = params.get("appid")?.to_owned();
        let is_public = params.get("is_public")?.parse::<bool>()?;

        // get the item from db, check whether obj in db
        let (sql, sql_params) = GutpSubspace::build_get_by_id(id);
        let rowset = pg::query(&pg_addr, &sql, &sql_params)?;
        match rowset.rows.next() {
            Some(row) => {
                let old_subspace = GutpSubspace::from_row(row);

                // TODO: update new obj with old
                let subspace = GutpSubspace {
                    title,
                    description,
                    banner,
                    owner_id,
                    profession,
                    appid,
                    is_public,
                    ..old_subspace
                };

                let (sql, sql_params) = subspace.build_update();
                _ = pg::execute(&pg_addr, &sql, &sql_params)?;

                let results: Vec<GutpSubspace> = vec![subspace];

                let info = Info {
                    model_name: GutpSubspace::model_name(),
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

        let (sql_statement, sql_params) = GutpSubspace::build_delete(id);
        _ = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: HandlerCRUD::Delete,
            extra: "".to_string(),
        };
        let results: Vec<GutpSubspace> = vec![];

        Ok(Response::new(Status::Successful, info, results))
    }
}

impl Module for GutpSubspaceModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/v1/subspace", Self::get_one);
        router.get("/v1/subspace/list", Self::get_list);
        router.get("/v1/subspace/list_by_owner", Self::list_by_owner);
        router.get("/v1/subspace/list_by_profession", Self::list_by_profession);
        router.get("/v1/subspace/list_by_appid", Self::list_by_appid);
        router.post("/v1/subspace/create", Self::new_one);
        router.post("/v1/subspace/update", Self::update);
        router.post("/v1/subspace/delete", Self::delete);

        Ok(())
    }
}
