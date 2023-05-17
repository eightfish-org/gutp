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

use gutp_types::GutpComment;

enum GutpCommentStatus {
    Normal = 0,
    Frozen = 1,
    Forbidden = 2,
    Deleted = 3,
}

enum GutpCommentWeight {
    Normal = 0,
}

pub struct GutpCommentModule;

impl GutpCommentModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;
        let comment_id = params.get("id")?;

        let (sql_statement, sql_params) = GutpComment::build_get_one_sql_and_params(comment_id);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let results = if let Some(row) = rowset.rows.next() {
            vec![GutpComment::from_row(row)]
        } else {
            return bail!("no this item".to_string());
        };

        let info = Info {
            model_name: GutpComment::model_name(),
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

        let (sql_statement, sql_params) = GutpComment::build_get_list_sql_and_params(offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let mut results: Vec<GutpComment> = vec![];
        for row in rowset.rows {
            let sp = GutpComment::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpComment::model_name(),
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
            GutpComment::build_get_list_by_sql_and_params("post_id", post_id, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let mut results: Vec<GutpComment> = vec![];
        for row in rowset.rows {
            let sp = GutpComment::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpComment::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn list_by_author(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let author_id = params.get("author_id")?;
        let page = params.get("page").unwrap_or(0);
        let limit = params.get("pagesize").unwrap_or(PAGESIZE);
        let offset = page * limit;

        let (sql_statement, sql_params) =
            GutpComment::build_get_list_by_sql_and_params("author_id", author_id, offset, limit);
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;

        let mut results: Vec<GutpComment> = vec![];
        for row in rowset.rows {
            let sp = GutpComment::from_row(row);
            results.push(sp);
        }

        let info = Info {
            model_name: GutpComment::model_name(),
            action: HandlerCRUD::List,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let content = params.get("content")?.to_owned();
        let author_id = params.get("author_id")?.to_owned();
        let post_id = params.get("post_id")?.to_owned();
        let parent_comment_id = params.get("parent_comment_id")?.to_owned();
        let is_public = params.get("is_public")?.parse::<bool>()?;

        let id = req.ext().get("random_str")?.to_owned();
        let time = req.ext().get("time")?.parse::<i64>()?;

        let comment = GutpComment {
            id,
            content,
            author_id,
            post_id,
            parent_comment_id,
            is_public,
            status: GutpCommentStatus::Normal,
            weight: GutpCommentWeight::Normal,
            created_time: time,
        };

        // construct a sql statement and param
        let (sql_statement, sql_params) = comment.build_insert_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

        let results: Vec<GutpComment> = vec![comment];

        let info = Info {
            model_name: GutpComment::model_name(),
            action: HandlerCRUD::Create,
            extra: "".to_string(),
        };

        Ok(Response::new(Status::Successful, info, results))
    }

    fn update(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;

        let params = req.parse_urlencoded()?;

        let id = params.get("id")?;
        let content = params.get("content")?.to_owned();
        let author_id = params.get("author_id")?.to_owned();
        let post_id = params.get("post_id")?.to_owned();
        let parent_comment_id = params.get("parent_comment_id")?.to_owned();
        let is_public = params.get("is_public")?.parse::<bool>()?;

        // get the item from db, check whether obj in db
        let (sql_statement, sql_params) = GutpComment::build_get_one_sql_and_params(id.as_str());
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params)?;
        match rowset.rows.next() {
            Some(row) => {
                let old_comment = GutpComment::from_row(row);

                let comment = GutpComment {
                    content,
                    author_id,
                    post_id,
                    parent_comment_id,
                    is_public,
                    ..old_comment
                };

                let (sql_statement, sql_params) = comment.build_update_sql_and_params();
                let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

                let results: Vec<GutpComment> = vec![comment];

                let info = Info {
                    model_name: GutpComment::model_name(),
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

        let (sql_statement, sql_params) = GutpComment::build_delete_sql_and_params(id.as_str());
        let _er = pg::execute(&pg_addr, &sql_statement, &sql_params)?;

        let info = Info {
            model_name: GutpComment::model_name(),
            action: HandlerCRUD::Delete,
            extra: "".to_string(),
        };
        let results: Vec<GutpComment> = vec![];

        Ok(Response::new(Status::Successful, info, results))
    }
}

impl Module for GutpCommentModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/v1/comment", Self::get_one);
        router.get("/v1/comment/list", Self::get_list);
        router.get("/v1/comment/list_by_post", Self::list_by_post);
        router.get("/v1/comment/list_by_author", Self::list_by_author);
        router.post("/v1/comment/create", Self::new_one);
        router.post("/v1/comment/update", Self::update);
        router.post("/v1/comment/delete", Self::delete);

        Ok(())
    }
}
