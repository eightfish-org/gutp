use crate::constants::{DB_URL, REDIS_URL};
use crate::{subspace, utils};
use anyhow::{anyhow, bail};
use eightfish_sdk::{EightFishModel, Module, Request, Response, Result, Router, StatusCode};
use gutp_types::{
    GutpPost, GutpPostStatus, GutpPostWeight, GutpSubspace, GutpSubspaceStatus, GutpSubspaceWeight,
    GutpUser, GutpUserRole, GutpUserStatus,
};
use serde_json::json;
use spin_sdk::pg::ParameterValue;
use spin_worker::{
    sql_create_one, sql_delete, sql_delete_one, sql_query, sql_query_one, sql_update,
    sql_update_one,
};
use sql_builder::SqlBuilder;

pub struct GutpPostModule;

impl GutpPostModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;
        let post_id = params.get("id").ok_or(anyhow!("id is required"))?;

        let ap = sql_query_one!(GutpPost, &post_id);
        match ap {
            Some(ap) => Ok(Response::new_check(vec![ap])),
            None => {
                bail!("error when get: no this item in db");
            }
        }
    }

    fn get_list(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpPost::model_name())
            .fields(&GutpPost::fields())
            .order_desc(GutpPost::created_time())
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_params: Vec<ParameterValue> = vec![];
        let posts = sql_query!(GutpPost, &sql, &sql_params);

        Ok(Response::new_check(posts))
    }

    fn get_list_by_subspace(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let subspace_id = params
            .get("subspace_id")
            .ok_or(anyhow!("subspace_id is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpPost::model_name())
            .fields(&GutpPost::fields())
            .and_where_eq(GutpPost::subspace_id(), "$1")
            .order_desc(GutpPost::created_time())
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_params = vec![ParameterValue::Str(subspace_id.clone())];
        let posts = sql_query!(GutpPost, &sql, &sql_params);

        Ok(Response::new_check(posts))
    }

    fn list_by_author(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let author_id = params
            .get("author_id")
            .ok_or(anyhow!("author_id is required"))?;
        let (limit, offset) = utils::build_page_info(&params)?;

        let sql = SqlBuilder::select_from(&GutpPost::model_name())
            .fields(&GutpPost::fields())
            .and_where_eq(GutpPost::author_id(), "$1")
            .order_desc(GutpPost::created_time())
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_params = vec![ParameterValue::Str(author_id.clone())];
        let posts = sql_query!(GutpPost, &sql, &sql_params);

        Ok(Response::new_check(posts))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
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
        let subspace_id = params
            .get("subspace_id")
            .ok_or(anyhow!("subspace_id is required"))?
            .to_owned();
        let ext_link = params
            .get("ext_link")
            .ok_or(anyhow!("ext_link is required"))?
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
            .get("timestamp")
            .ok_or(anyhow!("timestamp is required"))?
            .parse::<i64>()?;

        let post = GutpPost {
            id,
            title,
            content,
            author_id,
            subspace_id,
            ext_link,
            parent_post_id: None,
            is_public,
            status: GutpPostStatus::Normal as i16,
            weight: GutpPostWeight::Normal as i16,
            created_time: time,
            updated_time: time,
            data_source: "".to_string(),
        };
        match sql_create_one!(req, post) {
            Ok(ap) => Ok(Response::new_check(vec![ap])),
            Err(_) => {
                let json_result = json!({
                    "status": "failed",
                    "info": "Error when creating a new post",
                });
                Ok(Response::new_uncheck(StatusCode::BAD_REQUEST, json_result))
            }
        }
    }

    fn update(req: &mut Request) -> Result<Response> {
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
            .get("timestamp")
            .ok_or(anyhow!("timestamp is required"))?
            .parse::<i64>()?;

        let ap = sql_query_one!(GutpPost, &id);
        match ap {
            Some(old_ap) => {
                let ap = GutpPost {
                    title,
                    content,
                    author_id,
                    ext_link,
                    is_public,
                    updated_time: time,
                    ..old_ap
                };

                match sql_update_one!(req, ap) {
                    Ok(ap) => Ok(Response::new_check(vec![ap])),
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

        let item = sql_query_one!(GutpPost, &id);
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

impl Module for GutpPostModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/gutp/v1/post", Self::get_one);
        router.get("/gutp/v1/post/list", Self::get_list);
        router.get("/gutp/v1/post/list_by_subspace", Self::get_list_by_subspace);
        router.get("/gutp/v1/post/list_by_author", Self::list_by_author);
        router.post("/gutp/v1/post/create", Self::new_one);
        router.put("/gutp/v1/post/update", Self::update);
        router.delete("/gutp/v1/post/delete", Self::delete);

        Ok(())
    }
}
