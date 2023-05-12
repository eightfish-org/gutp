use eightfish::{EightFishModel, Info, Module, Request, Response, Result, Router, Status};
use eightfish_derive::EightFishModel;
use serde::{Deserialize, Serialize};
use spin_sdk::pg;

const REDIS_URL_ENV: &str = "REDIS_URL";
const DB_URL_ENV: &str = "DB_URL";

use crate::gutp_types::GutpSubspace;

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

pub struct SubspaceModule;

impl SubspaceModule {
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

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
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
        let private = params.get("private")?.parse::<bool>()?;

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
            private,
            status: GutpSubspaceStatus::Normal,
            weight: GutpSubspaceWeight::Normal,
            created_time: time,
        };

        // construct a sql statement and param
        let (sql_statement, sql_params) = subspace.build_insert_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params)?;
        println!(
            "in handler article_new: _execute_results: {:?}",
            _execute_results
        );

        let results: Vec<Article> = vec![article];

        let info = Info {
            model_name: GutpSubspace::model_name(),
            action: "new".to_string(),
            target: id.clone(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded();

        let id = params.get("id").unwrap();
        let title = params.get("title").unwrap();
        let content = params.get("content").unwrap();
        let authorname = params.get("authorname").unwrap();

        // construct a struct
        let article = Article {
            id: id.clone(),
            title: title.clone(),
            content: content.clone(),
            authorname: authorname.clone(),
        };

        // construct a sql statement and params
        let (sql_statement, sql_params) = article.build_update_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params);

        let results: Vec<Article> = vec![article];

        let info = Info {
            model_name: "article".to_string(),
            action: "update".to_string(),
            target: id.clone(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded();

        let id = params.get("id").unwrap();

        // construct a sql statement
        let (sql_statement, sql_params) = Article::build_delete_sql_and_params(id.as_str());
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params);
        // TODO check the pg result

        let results: Vec<Article> = vec![];

        let info = Info {
            model_name: "article".to_string(),
            action: "delete".to_string(),
            target: id.clone(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }
}

impl Module for ArticleModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/article/:id", Self::get_one);
        //router.get("/article/latest", Self::get_latest);
        router.post("/article/new", Self::new_article);
        router.post("/article/update", Self::update);
        router.post("/article/delete/:id", Self::delete);

        Ok(())
    }
}
