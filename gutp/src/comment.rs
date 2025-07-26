use crate::constants::{DB_URL, REDIS_URL};
use crate::{subspace, utils};
use anyhow::{anyhow, bail};
use eightfish_sdk::{EightFishModel, Module, Request, Response, Result, Router, StatusCode};
use gutp_types::{
    GutpComment, GutpCommentStatus, GutpCommentWeight, GutpPost, GutpPostStatus, GutpPostWeight,
    GutpSubspace, GutpSubspaceStatus, GutpSubspaceWeight, GutpUser, GutpUserRole, GutpUserStatus,
};
use serde_json::json;
use spin_sdk::pg::ParameterValue;
use spin_worker::{
    sql_create_one, sql_delete, sql_delete_one, sql_query, sql_query_one, sql_update,
    sql_update_one,
};
use sql_builder::SqlBuilder;

pub struct GutpCommentModule;

impl GutpCommentModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;
        let id = params.get("id").ok_or(anyhow!("id required."))?;

        let item = sql_query_one!(GutpComment, &id);
        match item {
            Some(item) => Ok(Response::new_check(vec![item])),
            None => {
                bail!("error when get: no this item in db");
            }
        }
    }

    fn get_list(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpComment::model_name())
            .fields(&GutpComment::fields())
            .order_desc(GutpComment::created_time())
            .limit(limit)
            .offset(offset)
            .sql()?;

        let sql_params: Vec<ParameterValue> = vec![];
        let results = sql_query!(GutpComment, &sql, &sql_params);

        Ok(Response::new_check(results))
    }

    fn list_by_post(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let post_id = params
            .get("post_id")
            .ok_or(anyhow!("post_id is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpComment::model_name())
            .fields(&GutpComment::fields())
            .and_where_eq(GutpComment::post_id(), "$1")
            .order_desc(GutpComment::created_time())
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_params = vec![ParameterValue::Str(post_id.clone())];
        let results = sql_query!(GutpComment, &sql, &sql_params);

        Ok(Response::new_check(results))
    }

    fn list_by_author(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let author_id = params
            .get("author_id")
            .ok_or(anyhow!("author_id is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpComment::model_name())
            .fields(&GutpComment::fields())
            .and_where_eq(GutpComment::author_id(), "$1")
            .order_desc(GutpComment::created_time())
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_params = vec![ParameterValue::Str(author_id.clone())];
        let results = sql_query!(GutpComment, &sql, &sql_params);

        Ok(Response::new_check(results))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let content = params
            .get("content")
            .ok_or(anyhow!("content is required."))?
            .to_owned();
        let author_id = params
            .get("author_id")
            .ok_or(anyhow!("author_id is required."))?
            .to_owned();
        let post_id = params
            .get("post_id")
            .ok_or(anyhow!("post_id required."))?
            .to_owned();
        let is_public = params
            .get("is_public")
            .ok_or(anyhow!("is_public is required."))?
            .parse::<bool>()?;
        let parent_comment_id: Option<String> =
            params.get("parent_comment_id").map(|s| s.to_owned());

        let id = req
            .ext()
            .get("random_str")
            .ok_or(anyhow!("generate id failed"))?
            .to_owned();
        let time = req
            .ext()
            .get("time")
            .ok_or(anyhow!("generate time failed"))?
            .parse::<i64>()?;

        let comment = GutpComment {
            id,
            content,
            author_id,
            post_id,
            parent_comment_id,
            is_public,
            status: GutpCommentStatus::Normal as i16,
            weight: GutpCommentWeight::Normal as i32,
            created_time: time,
            data_source: "".to_string(),
        };

        match sql_create_one!(req, comment) {
            Ok(item) => Ok(Response::new_check(vec![item])),
            Err(_) => {
                let json_result = json!({
                    "status": "failed",
                    "info": "Error when creating a new comment",
                });
                Ok(Response::new_uncheck(StatusCode::BAD_REQUEST, json_result))
            }
        }
    }

    fn update(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let id = params.get("id").ok_or(anyhow!("id is required."))?;
        let content = params
            .get("content")
            .ok_or(anyhow!("content is required."))?
            .to_owned();
        let is_public = params
            .get("is_public")
            .ok_or(anyhow!("is_public is required."))?
            .parse::<bool>()?;

        let item = sql_query_one!(GutpComment, &id);
        match item {
            Some(old_comment) => {
                let comment = GutpComment {
                    content,
                    is_public,
                    ..old_comment
                };

                match sql_update_one!(req, comment) {
                    Ok(item) => Ok(Response::new_check(vec![item])),
                    Err(_) => {
                        bail!("update error: db operation error")
                    }
                }
            }
            None => {
                bail!("update error: no this item in db");
            }
        }
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;
        let id = params.get("id").ok_or(anyhow!("id is required"))?;

        let item = sql_query_one!(GutpComment, &id);
        match item {
            Some(item) => match sql_delete_one!(req, item) {
                Ok(item) => Ok(Response::new_check(vec![item])),
                Err(_) => {
                    bail!("delete error: db operation error")
                }
            },
            None => {
                bail!("delete error: no this item in db");
            }
        }
    }
}

impl Module for GutpCommentModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/gutp/v1/comment", Self::get_one);
        router.get("/gutp/v1/comment/list", Self::get_list);
        router.get("/gutp/v1/comment/list_by_post", Self::list_by_post);
        router.get("/gutp/v1/comment/list_by_author", Self::list_by_author);
        router.post("/gutp/v1/comment/create", Self::new_one);
        router.put("/gutp/v1/comment/update", Self::update);
        router.delete("/gutp/v1/comment/delete", Self::delete);

        Ok(())
    }
}
