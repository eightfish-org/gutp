use crate::constants::{DB_URL, REDIS_URL};
use crate::{subspace, utils};
use anyhow::{anyhow, bail};
use eightfish_sdk::{EightFishModel, Module, Request, Response, Result, Router, StatusCode};
use gutp_types::{
    GutpSubspace, GutpSubspaceStatus, GutpSubspaceWeight, GutpUser, GutpUserRole, GutpUserStatus,
};
use serde_json::json;
use spin_sdk::pg::ParameterValue;
use spin_worker::{
    sql_create_one, sql_delete, sql_delete_one, sql_query, sql_query_one, sql_update,
    sql_update_one,
};
use sql_builder::SqlBuilder;

pub struct GutpSubspaceModule;

impl GutpSubspaceModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let subspace_id = params.get("id").ok_or(anyhow!("no id"))?;

        let sp = sql_query_one!(GutpSubspace, &subspace_id);
        match sp {
            Some(sp) => {
                let results: Vec<GutpSubspace> = vec![sp];

                Ok(Response::new_check(results))
            }
            None => {
                bail!("get subspace: no subspace in db");
            }
        }
    }

    fn get_list(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let (limit, offset) = utils::build_page_info(&params)?;
        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .order_desc(GutpSubspace::created_time())
            .limit(limit)
            .offset(offset)
            .sql()?;

        let sql_params: Vec<ParameterValue> = vec![];
        let sps = sql_query!(GutpSubspace, &sql, &sql_params);

        Ok(Response::new_check(sps))
    }

    fn list_by_owner(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let owner_id = params
            .get("owner_id")
            .ok_or(anyhow!("owner_id is required"))?;

        let (limit, offset) = utils::build_page_info(&params)?;
        let sql = SqlBuilder::select_from(&GutpSubspace::model_name())
            .fields(&GutpSubspace::fields())
            .and_where_eq(GutpSubspace::owner_id(), "$1")
            .order_desc(GutpSubspace::created_time())
            .limit(limit)
            .offset(offset)
            .sql()?;
        let sql_params = vec![ParameterValue::Str(owner_id.clone())];
        let sps = sql_query!(GutpSubspace, &sql, &sql_params);

        Ok(Response::new_check(sps))
    }

    fn new_one(req: &mut Request) -> Result<Response> {
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
        let is_public = params
            .get("is_public")
            .ok_or(anyhow!("is_public is required"))?
            .parse::<bool>()?;
        let owner_id: Option<String> = params.get("owner_id").map(|s| s.to_owned());

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
            created_time: time,
            data_source: "".to_string(),
        };
        match sql_create_one!(req, subspace) {
            Ok(sp) => {
                let results: Vec<GutpSubspace> = vec![sp];

                Ok(Response::new_check(results))
            }
            Err(_) => {
                let json_result = json!({
                    "status": "failed",
                    "info": "Error when creating a new subspace",
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
        let is_public = params
            .get("is_public")
            .ok_or(anyhow!("is_public is required"))?
            .parse::<bool>()?;
        // let time = req
        //     .ext()
        //     .get("time")
        //     .ok_or(anyhow!("time is required"))?
        //     .parse::<i64>()?;

        let sp = sql_query_one!(GutpSubspace, &id);
        match sp {
            Some(old_sp) => {
                let subspace = GutpSubspace {
                    title,
                    description,
                    banner,
                    owner_id: Some(owner_id),
                    is_public,
                    ..old_sp
                };

                match sql_update_one!(req, subspace) {
                    Ok(sp) => Ok(Response::new_check(vec![sp])),
                    Err(_) => {
                        bail!("update subspace error: db operation error")
                    }
                }
            }
            None => {
                bail!("update subspace error: no this subspace in db");
            }
        }
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;
        let id = params.get("id").ok_or(anyhow!("id is required"))?;

        let sp = sql_query_one!(GutpSubspace, &id);
        match sp {
            Some(sp) => match sql_delete_one!(req, sp) {
                Ok(sp) => Ok(Response::new_check(vec![sp])),
                Err(_) => {
                    bail!("delete subspace error: db operation error")
                }
            },
            None => {
                bail!("delete subspace error: no this item in db");
            }
        }
    }
}

impl Module for GutpSubspaceModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/gutp/v1/subspace", Self::get_one);
        router.get("/gutp/v1/subspace/list", Self::get_list);
        router.get("/gutp/v1/subspace/list_by_owner", Self::list_by_owner);
        router.post("/gutp/v1/subspace/create", Self::new_one);
        router.put("/gutp/v1/subspace/update", Self::update);
        router.delete("/gutp/v1/subspace/delete", Self::delete);

        Ok(())
    }
}
