use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{postgres::Postgres, redis::Redis, schema};

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::projects)]
struct PartialProject {
    id: i32,
    name: String,
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::projects)]
struct CompleteProject {
    id: i32,
    name: String,
    description: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct NewProjectRequest {
    name: String,
    description: String,
}

#[derive(Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::projects)]
struct NewProject {
    name: String,
    description: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ModifyProjectRequest {
    name: String,
    description: String,
}

#[derive(Serialize, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::projects)]
struct ModifyProject {
    name: String,
    description: String,
    updated_at: NaiveDateTime,
}

pub async fn list_projects(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::projects::table
        .select(PartialProject::as_select())
        .load::<PartialProject>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(projects)) => {
            return HttpResponse::Ok().json(projects);
        }
        Ok(None) => {
            return HttpResponse::Ok().json(Vec::<PartialProject>::new());
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_project_by_id(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let result = schema::projects::table
        .select(CompleteProject::as_select())
        .filter(schema::projects::id.eq(search_id.into_inner()))
        .first::<CompleteProject>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(project)) => {
            return HttpResponse::Ok().json(project);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_project_by_name(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_name: web::Path<String>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::projects::table
        .select(CompleteProject::as_select())
        .filter(schema::projects::name.eq(&search_name.into_inner()))
        .first::<CompleteProject>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(project)) => {
            return HttpResponse::Ok().json(project);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn create_project(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    create_project_request: web::Json<NewProjectRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let create_project = NewProject {
        name: create_project_request.name.clone(),
        description: create_project_request.description.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let count = diesel::insert_into(schema::projects::table)
        .values(&create_project)
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Created().finish();
        }
        Ok(None) => {
            return HttpResponse::InternalServerError().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn update_project(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    update_id: web::Path<i32>,
    update_project_request: web::Json<ModifyProjectRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let update_project = ModifyProject {
        name: update_project_request.name.clone(),
        description: update_project_request.description.clone(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let count = diesel::update(schema::projects::table)
        .set(&update_project)
        .filter(schema::projects::id.eq(update_id.into_inner()))
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn delete_project(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    delete_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let count = diesel::delete(schema::projects::table)
        .filter(schema::projects::id.eq(delete_id.into_inner()))
        .returning((
            schema::projects::id,
            schema::projects::name,
            schema::projects::description,
        ))
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}
