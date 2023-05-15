use eightfish::{EightFishModel, Info, Module, Request, Response, Result, Router, Status};
use eightfish_derive::EightFishModel;
use serde::{Deserialize, Serialize};
use spin_sdk::pg;

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
        // println!("in handler subspace get_one: params: {:?}", params);

        let subspace_id = params.get("id")?;

        // construct a sql statement
        let (sql_statement, sql_params) =
            GutpSubspace::build_get_one_sql_and_params(subspace_id.as_str());
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        // println!("in handler subspace get_one: rowset: {:?}", rowset);

        // convert the raw vec[u8] to every rust struct filed, and convert the whole into a
        // rust struct vec, later we may find a gerneral type converter way
        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }
        // println!("in handler subspace get_one: results: {:?}", results);

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: "get_one".to_string(),
            target: subspace_id.clone(),
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn get_list(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        // println!("in handler subspace get_one: params: {:?}", params);

        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        // construct a sql statement
        let (sql_statement, sql_params) =
            GutpSubspace::build_get_list_sql_and_params(offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        // println!("in handler subspace get_one: rowset: {:?}", rowset);

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }
        // println!("in handler subspace get_one: results: {:?}", results);

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: "get_one".to_string(),
            target: subspace_id.clone(),
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_owner(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        // println!("in handler subspace get_one: params: {:?}", params);

        let owner = params.get("owner")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        // construct a sql statement
        let (sql_statement, sql_params) =
            GutpSubspace::build_get_list_by_sql_and_params("owner", owner, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        // println!("in handler subspace get_one: rowset: {:?}", rowset);

        // convert the raw vec[u8] to every rust struct filed, and convert the whole into a
        // rust struct vec, later we may find a gerneral type converter way
        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }
        // println!("in handler subspace get_one: results: {:?}", results);

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: "get_one".to_string(),
            target: subspace_id.clone(),
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

        let (sql_statement, sql_params) =
            GutpSubspace::build_get_list_by_sql_and_params("profession", profession, offset, limit);

        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        // println!("in handler subspace get_one: rowset: {:?}", rowset);

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }
        // println!("in handler subspace get_one: results: {:?}", results);

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: "get_one".to_string(),
            target: subspace_id.clone(),
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_appid(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        // println!("in handler subspace get_one: params: {:?}", params);

        let appid = params.get("appid")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let (sql_statement, sql_params) =
            GutpSubspace::build_get_list_by_sql_and_params("appid", appid, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        // println!("in handler subspace get_one: rowset: {:?}", rowset);

        let mut results: Vec<GutpSubspace> = vec![];
        for row in rowset.rows {
            let sp = GutpSubspace::from_row(row);
            results.push(sp);
        }
        // println!("in handler subspace get_one: results: {:?}", results);

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: "get_one".to_string(),
            target: subspace_id.clone(),
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
        let owner = params.get("owner")?.to_owned();
        let profession = params.get("profession")?.to_owned();
        let appid = params.get("appid")?.to_owned();
        let is_public = params.get("is_public")?.parse::<bool>()?;

        let id = req.ext().get("random_str")?.to_owned();
        let time = req.ext().get("time")?.parse::<i64>()?;

        // construct a struct
        let subspace = GutpSubspace {
            id,
            title,
            description,
            banner,
            owner,
            profession,
            appid,
            is_public,
            status: GutpSubspaceStatus::Normal,
            weight: GutpSubspaceWeight::Normal,
            created_time: time,
        };

        // construct a sql statement and param
        let (sql_statement, sql_params) = subspace.build_insert_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

        let results: Vec<GutpSubspace> = vec![subspace];

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: "new".to_string(),
            target: id.clone(),
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
        let owner = params.get("owner")?.to_owned();
        let profession = params.get("profession")?.to_owned();
        let appid = params.get("appid")?.to_owned();
        let is_public = params.get("is_public")?.parse::<bool>()?;

        // get the item from db, check whether obj in db
        let (sql_statement, sql_params) = GutpSubspace::build_get_one_sql_and_params(id.as_str());
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        match rowset.rows.next() {
            Some(row) => {
                let old_subspace = GutpSubspace::from_row(row);

                // TODO: update new obj with old
                let subspace = GutpSubspace {
                    title,
                    description,
                    banner,
                    owner,
                    profession,
                    appid,
                    is_public,
                    ..old_subspace
                };

                let (sql_statement, sql_params) = subspace.build_update_sql_and_params();
                let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

                let results: Vec<GutpSubspace> = vec![subspace];

                let info = Info {
                    model_name: GutpSubspace::model_name(),
                    action: "update".to_string(),
                    target: id.to_owned(),
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

        // construct a sql statement
        let (sql_statement, sql_params) = GutpSubspace::build_delete_sql_and_params(id.as_str());
        let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: "delete".to_string(),
            target: id.to_owned(),
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
