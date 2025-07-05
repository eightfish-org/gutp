use crate::constants::DB_URL_ENV;
use crate::utils;
use anyhow::{anyhow, bail};
use eightfish_sdk::{Module, Request, Response, Result, Router};
use spin_sdk::pg::{self, ParameterValue};
use sql_builder::SqlBuilder;

use gutp_types::GutpPost;

enum GutpPostStatus {
    Normal = 0,
    Frozen = 1,
    Forbidden = 2,
    Deleted = 3,
}

enum GutpPostWeight {
    Normal = 0,
    Low = -1,
    VeryLow = -2,
    SuperLow = -3,
    High = 1,
    VeryHigh = 2,
    SuperHigh = 3,
}

// PostWithInfo struct removed to avoid complexity - can be re-added later if needed



pub struct GutpPostModule;

impl GutpPostModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;
        let post_id = params.get("id").ok_or(anyhow!("id is required"))?;

        let (sql, sql_params) = GutpPost::build_get_by_id(post_id);
        let rowset = pg_conn.query(&sql, &sql_params)?;

        let results = if let Some(row) = rowset.rows.into_iter().next() {
            vec![GutpPost::from_row(row)]
        } else {
            bail!("no this item".to_string());
        };

        Ok(Response::new_check(results))
    }

    fn get_list(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpPost::model_name())
            .fields(&GutpPost::fields())
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let rowset = pg_conn.query(&sql, &[])?;

        let mut results: Vec<GutpPost> = vec![];
        for row in rowset.rows {
            let sp = GutpPost::from_row(row);
            results.push(sp);
        }

        Ok(Response::new_check(results))
    }

    fn get_list_by_subspace(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let subspace_id = params
            .get("subspace_id")
            .ok_or(anyhow!("subspace_id is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpPost::model_name())
            .fields(&GutpPost::fields())
            .and_where_eq("subspace_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(subspace_id.clone());
        let rowset = pg_conn.query(&sql, &[sql_param])?;

        let mut results: Vec<GutpPost> = vec![];
        for row in rowset.rows {
            let post = GutpPost::from_row(row);
            results.push(post);
        }

        Ok(Response::new_check(results))
    }

    fn list_by_author(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let author_id = params
            .get("author_id")
            .ok_or(anyhow!("author_id is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpPost::model_name())
            .fields(&GutpPost::fields())
            .and_where_eq("author_id", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(author_id.clone());
        let rowset = pg_conn.query(&sql, &[sql_param])?;

        let mut results: Vec<GutpPost> = vec![];
        for row in rowset.rows {
            let sp = GutpPost::from_row(row);
            results.push(sp);
        }

        Ok(Response::new_check(results))
    }

    fn list_by_category(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let category = params
            .get("category")
            .ok_or(anyhow!("category is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpPost::model_name())
            .fields(&GutpPost::fields())
            .and_where_eq("category", "$1")
            .order_desc("created_time")
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_param = ParameterValue::Str(category.clone());
        let rowset = pg_conn.query(&sql, &[sql_param])?;

        let mut results: Vec<GutpPost> = vec![];
        for row in rowset.rows {
            let sp = GutpPost::from_row(row);
            results.push(sp);
        }

        Ok(Response::new_check(results))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV)?;
        let pg_conn = pg::Connection::open(&pg_addr)?;

        let params = req.parse_urlencoded()?;

        let title = params
            .get("title")
            .ok_or(anyhow!("title is required"))?
            .to_owned();
        let content = params
            .get("content")
            .ok_or(anyhow!("content is required"))?
            .to_owned();
        let author_id = params
            .get("author_id")
            .ok_or(anyhow!("author_id is required"))?
            .to_owned();
        let author_nickname = params
            .get("author_nickname")
            .ok_or(anyhow!("author_nickname is required"))?
            .to_owned();
        let subspace_id = params
            .get("subspace_id")
            .ok_or(anyhow!("subspace_id is required"))?
            .to_owned();
        let ext_link = params
            .get("ext_link")
            .ok_or(anyhow!("ext_link is required"))?
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

        let post = GutpPost {
            id,
            title,
            content,
            author_id,
            author_nickname,
            subspace_id,
            ext_link,
            category,
            app_id,
            parent_post_id: "".to_string(),
            is_public,
            status: GutpPostStatus::Normal as i16,
            weight: GutpPostWeight::Normal as i16,
            created_time: time,
            updated_time: time,
        };

        let (sql_statement, sql_params) = post.build_insert();
        _ = pg_conn.execute(&sql_statement, &sql_params)?;

        let results: Vec<GutpPost> = vec![post];

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
        let content = params
            .get("content")
            .ok_or(anyhow!("contnet is required"))?
            .to_owned();
        let author_id = params
            .get("author_id")
            .ok_or(anyhow!("author_id is required"))?
            .to_owned();
        let ext_link = params
            .get("ext_link")
            .ok_or(anyhow!("ext_link is required"))?
            .to_owned();
        let is_public = params
            .get("is_public")
            .ok_or(anyhow!("is_public is required"))?
            .parse::<bool>()?;
        let time = req
            .ext()
            .get("time")
            .ok_or(anyhow!("time is required"))?
            .parse::<i64>()?;
        // get the item from db, check whether obj in db
        let (sql, sql_params) = GutpPost::build_get_by_id(id);
        let rowset = pg_conn.query(&sql, &sql_params)?;
        match rowset.rows.into_iter().next() {
            Some(row) => {
                let old_post = GutpPost::from_row(row);

                let post = GutpPost {
                    title,
                    content,
                    author_id,
                    ext_link,
                    is_public,
                    updated_time: time,
                    ..old_post
                };

                let (sql, sql_params) = post.build_update();
                _ = pg_conn.execute(&sql, &sql_params)?;

                let results: Vec<GutpPost> = vec![post];

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

        let post = req.parse_json_required::<GutpPost>()?;

        let (sql, sql_params) = post.build_delete();
        _ = pg_conn.execute(&sql, &sql_params)?;

        let results: Vec<GutpPost> = vec![];

        Ok(Response::new_check(results))
    }
}

impl Module for GutpPostModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/gutp/v1/post", Self::get_one);
        router.get("/gutp/v1/post/list", Self::get_list);
        router.get("/gutp/v1/post/list_by_subspace", Self::get_list_by_subspace);
        router.get("/gutp/v1/post/list_by_author", Self::list_by_author);
        router.get("/gutp/v1/post/list_by_category", Self::list_by_category);
        router.post("/gutp/v1/post/create", Self::new_one);
        router.post("/gutp/v1/post/update", Self::update);
        router.post("/gutp/v1/post/delete", Self::delete);

        Ok(())
    }
}
