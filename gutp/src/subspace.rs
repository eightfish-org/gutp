use anyhow::{anyhow, bail};
use eightfish_sdk::{Module, Request, Response, Result, Router};
use spin_sdk::pg::{self, ParameterValue};
use sql_builder::SqlBuilder;

use crate::constants::DB_URL_ENV;

use crate::utils;
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
    SuperHigh = 3,
}

pub struct GutpSubspaceModule;

impl GutpSubspaceModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let subspace_id = params.get("id").ok_or(anyhow!("no id"))?;

        let (sql, sql_params) = GutpSubspace::build_get_by_id(subspace_id);
        let rowset = pg_conn.query(&sql, &sql_params)?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }
        // println!("in handler subspace get_one: results: {:?}", results);

        Ok(Response::new_check(results))
    }

    fn get_list(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let (limit, offset) = utils::build_page_info(&params)?;
        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let rowset = pg_conn.query(&sql, &[])?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }

        Ok(Response::new_check(results))
    }

    fn list_by_owner(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;
        // println!("in handler subspace get_one: params: {:?}", params);

        let owner_id = params
            .get("owner_id")
            .ok_or(anyhow!("owner_id is required"))?;

        let (limit, offset) = utils::build_page_info(&params)?;
        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .and_where_eq("owner_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(owner_id.clone());
        let rowset = pg_conn.query(&sql, &[sql_param])?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }

        Ok(Response::new_check(results))
    }

    fn list_by_category(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;
        // println!("in handler subspace get_one: params: {:?}", params);

        let category = params
            .get("category")
            .ok_or(anyhow!("category is required"))?;

        let (limit, offset) = utils::build_page_info(&params)?;
        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .and_where_eq("category", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(category.clone());
        let rowset = pg_conn.query(&sql, &[sql_param])?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }

        Ok(Response::new_check(results))
    }

    fn list_by_appid(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let app_id = params
            .get("app_id")
            .ok_or(anyhow!("app_id is required"))?;

        let (limit, offset) = utils::build_page_info(&params)?;
        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .and_where_eq("app_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(app_id.clone());
        let rowset = pg_conn.query(&sql, &[sql_param])?;

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }

        Ok(Response::new_check(results))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let slug = params
            .get("slug")
            .ok_or(anyhow!("slug is required"))?
            .to_owned();
        let title = params
            .get("title")
            .ok_or(anyhow!("title is required"))?
            .to_owned();
        let description = params
            .get("description")
            .ok_or(anyhow!("description is required"))?
            .to_owned();
        let banner = params
            .get("banner")
            .ok_or(anyhow!("banner is required"))?
            .to_owned();
        let owner_id = params
            .get("owner_id")
            .ok_or(anyhow!("owner_id is required"))?
            .to_owned();
        let category = params
            .get("category")
            .ok_or(anyhow!("category is required"))?
            .to_owned();
        let app_id = params
            .get("app_id")
            .ok_or(anyhow!("app_id is required"))?
            .to_owned();
        let is_public = params
            .get("is_public")
            .ok_or(anyhow!("is_public is required"))?
            .parse::<bool>()?;

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

        let subspace = GutpSubspace {
            id,
            slug,
            title,
            description,
            banner,
            is_public,
            status: GutpSubspaceStatus::Normal as i16,
            weight: GutpSubspaceWeight::Normal as i16,
            owner_id,
            category,
            app_id,
            created_time: time,
        };

        let (sql, sql_params) = subspace.build_insert();
        _ = pg_conn.execute(&sql, &sql_params)?;

        let results: Vec<GutpSubspace> = vec![subspace];

        Ok(Response::new_check(results))
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id").ok_or(anyhow!("id is required"))?;
        let title = params
            .get("title")
            .ok_or(anyhow!("title is required"))?
            .to_owned();
        let description = params
            .get("description")
            .ok_or(anyhow!("description is required"))?
            .to_owned();
        let banner = params
            .get("banner")
            .ok_or(anyhow!("banner is required"))?
            .to_owned();
        let owner_id = params
            .get("owner_id")
            .ok_or(anyhow!("owner_id is required"))?
            .to_owned();
        let category = params
            .get("category")
            .ok_or(anyhow!("profession is required"))?
            .to_owned();
        let app_id = params
            .get("app_id")
            .ok_or(anyhow!("appid is required"))?
            .to_owned();
        let is_public = params
            .get("is_public")
            .ok_or(anyhow!("is_public is required"))?
            .parse::<bool>()?;
        // let time = req
        //     .ext()
        //     .get("time")
        //     .ok_or(anyhow!("time is required"))?
        //     .parse::<i64>()?;

        // get the item from db, check whether obj in db
        let (sql, sql_params) = GutpSubspace::build_get_by_id(id);
        let rowset = pg_conn.query(&sql, &sql_params)?;
        match rowset.rows.into_iter().next() {
            Some(row) => {
                let old_subspace = GutpSubspace::from_row(row);

                // TODO: update new obj with old
                let subspace = GutpSubspace {
                    title,
                    description,
                    banner,
                    owner_id,
                    category,
                    app_id,
                    is_public,
                    ..old_subspace
                };

                let (sql, sql_params) = subspace.build_update();
                _ = pg_conn.execute(&sql, &sql_params)?;

                let results: Vec<GutpSubspace> = vec![subspace];

                Ok(Response::new_check(results))
            }
            None => {
                bail!("update action: no item in db");
            }
        }
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let subspace = req.parse_json_required::<GutpSubspace>()?;

        let (sql_statement, sql_params) = subspace.build_delete();
        _ = pg_conn.execute(&sql_statement, &sql_params)?;

        let results: Vec<GutpSubspace> = vec![];

        Ok(Response::new_check(results))
    }
}

impl Module for GutpSubspaceModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/gutp/v1/subspace", Self::get_one);
        router.get("/gutp/v1/subspace/list", Self::get_list);
        router.get("/gutp/v1/subspace/list_by_owner", Self::list_by_owner);
        router.get("/gutp/v1/subspace/list_by_category", Self::list_by_category);
        router.get("/gutp/v1/subspace/list_by_appid", Self::list_by_appid);
        router.post("/gutp/v1/subspace/create", Self::new_one);
        router.post("/gutp/v1/subspace/update", Self::update);
        router.post("/gutp/v1/subspace/delete", Self::delete);

        Ok(())
    }
}
