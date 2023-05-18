use eightfish::{EightFishModel, Info, Module, Request, Response, Result, Router, Status};
use eightfish_derive::EightFishModel;
use gutp_types::GutpUser;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{self, DbValue, Decode, ParameterValue};

const REDIS_URL_ENV: &str = "REDIS_URL";
const DB_URL_ENV: &str = "DB_URL";

// pub struct GutpUser {
//     id: String,
//     account: String,
//     nickname: String,
//     avatar: String,
//     role: i64,
//     status: i64,
//     signup_time: i64,
//     pub_settings: String,
//     ext: String,
// }

pub struct GutpUserModule;

impl GutpUserModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded();
        println!("in handler user params: {:?}", params);

        let entity_id = params.get("id").unwrap();

        // construct a sql statement
        let (sql_statement, sql_params) =
            GutpUser::build_get_one_sql_and_params(entity_id.as_str());
        let rowset = pg::query(&pg_addr, &sql_statement, &sql_params).unwrap();
        println!("in handler user rowset: {:?}", rowset);

        // convert the raw vec[u8] to every rust struct filed, and convert the whole into a
        // rust struct vec, later we may find a gerneral type converter way
        let mut results: Vec<GutpUser> = vec![];
        for row in rowset.rows {
            let article = GutpUser::from_row(row);
            results.push(article);
        }
        println!("in handler user results: {:?}", results);

        let info = Info {
            model_name: GutpUser::model_name(),
            action: "get_one".to_string(),
            target: entity_id.clone(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }

    fn new_user(req: &mut Request) -> Result<Response> {
        let pg_addr = std::env::var(DB_URL_ENV).unwrap();

        let params = req.parse_urlencoded();

        let account = params.get("account").unwrap();
        let nickname = params.get("nickname").unwrap();
        let avatar = params.get("avatar").unwrap();
        let role = params.get("role").unwrap();
        let status = params.get("status").unwrap();
        let pub_settings = params.get("pub_settings").unwrap();
        let ext = params.get("ext").unwrap();

        let id = req.ext().get("random_str").unwrap();

        // construct a struct
        let article = GutpUser {
            id: id.clone(),
            account: account.clone(),
            nickname: nickname.clone(),
            avatar: avatar.clone(),
            role: role.parse::<i64>().unwrap(),
            status: status.parse::<i64>().unwrap(),
            signup_time: 0,
            pub_settings: pub_settings.clone(),
            ext: ext.clone(),
        };

        // construct a sql statement and param
        let (sql_statement, sql_params) = article.build_insert_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params);
        println!(
            "in handler article_new: _execute_results: {:?}",
            _execute_results
        );

        let results: Vec<GutpUser> = vec![article];

        let info = Info {
            model_name: GutpUser::model_name(),
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
        let account = params.get("account").unwrap();
        let nickname = params.get("nickname").unwrap();
        let avatar = params.get("avatar").unwrap();
        let role = params.get("role").unwrap();
        let status = params.get("status").unwrap();
        let pub_settings = params.get("pub_settings").unwrap();
        let ext = params.get("ext").unwrap();

        // construct a struct
        let article = GutpUser {
            id: id.clone(),
            account: account.clone(),
            nickname: nickname.clone(),
            avatar: avatar.clone(),
            role: role.parse::<i64>().unwrap(),
            status: status.parse::<i64>().unwrap(),
            signup_time: 0,
            pub_settings: pub_settings.clone(),
            ext: ext.clone(),
        };

        // construct a sql statement and params
        let (sql_statement, sql_params) = article.build_update_sql_and_params();
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params);

        let results: Vec<GutpUser> = vec![article];

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
        let (sql_statement, sql_params) = GutpUser::build_delete_sql_and_params(id.as_str());
        let _execute_results = pg::execute(&pg_addr, &sql_statement, &sql_params);
        // TODO check the pg result

        let results: Vec<GutpUser> = vec![];

        let info = Info {
            model_name: GutpUser::model_name(),
            action: "delete".to_string(),
            target: id.clone(),
            extra: "".to_string(),
        };

        let response = Response::new(Status::Successful, info, results);

        Ok(response)
    }
}

impl Module for GutpUserModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/v1/user/:id", Self::get_one);
        router.post("/v1/user/new", Self::new_user);
        router.post("/v1/user/update", Self::update);
        router.post("/v1/user/delete/:id", Self::delete);

        Ok(())
    }
}
