use actix_web::web;
use crate::handlers::projects::{list_projects, get_project_by_id, get_project_by_name, create_project, update_project, delete_project};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .route("", web::get().to(list_projects))
            .route("/{id}", web::get().to(get_project_by_id))
            .route("/name/{name}", web::get().to(get_project_by_name))
            .route("", web::post().to(create_project))
            .route("/{id}", web::put().to(update_project))
            .route("/{id}", web::delete().to(delete_project))
    );
}