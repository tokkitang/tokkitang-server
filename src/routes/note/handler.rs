use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use futures::future::join_all;

use crate::{
    extensions::CurrentUser,
    middlewares::auth,
    models::{InsertUser, Note, Project, Team, TeamUser, TeamUserAuthority, User},
    routes::{auth::AuthService, project::ProjectService, team::TeamService, user::UserService},
    utils::{generate_uuid, hash_password, AllError},
};

use super::{
    dto::{CreateNoteRequest, CreateNoteResponse},
    NoteService,
};

pub async fn router() -> Router {
    let app = Router::new().route("/", post(create_note));

    app
}

async fn create_note(
    current_user: Extension<CurrentUser>,
    database: Extension<Arc<Client>>,
    Json(body): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    let user = if let Some(user) = current_user.user.clone() {
        user
    } else {
        return (StatusCode::UNAUTHORIZED).into_response();
    };

    let team_service = TeamService::new(database.clone());
    let note_service = NoteService::new(database.clone());
    let project_service = ProjectService::new(database.clone());

    let mut response = CreateNoteResponse {
        success: false,
        note_id: "".into(),
    };

    let project = match project_service
        .get_project_by_id(body.project_id.clone())
        .await
    {
        Ok(project) => project,
        Err(error) => {
            if let AllError::NotFound = error {
                println!("# 프로젝트 없음");
                return (StatusCode::NOT_FOUND).into_response();
            } else {
                println!("error: {:?}", error);
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    };

    let team_id = project.team_id.clone();

    match team_service
        .find_team_user_by_team_and_user_id(team_id.clone(), user.id.clone())
        .await
    {
        Ok(Some(team_user)) => match team_user.authority {
            TeamUserAuthority::Owner | TeamUserAuthority::Admin | TeamUserAuthority::Write => {
                println!("# 권한 허용: OWNER OR ADMIN OR WRITE");
            }
            _ => {
                println!("# 권한 부족: NEED WRITE");
                return (StatusCode::FORBIDDEN).into_response();
            }
        },
        Ok(None) => {
            println!("# 권한 부족: NOT TEAM MEMBER");
            return (StatusCode::FORBIDDEN).into_response();
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    let data = Note {
        id: uuid::Uuid::new_v4().to_string(),
        project_id: body.project_id.clone(),
        content: body.content.clone(),
        x: body.x,
        y: body.y,
    };

    match note_service.create_note(data).await {
        Ok(note_id) => {
            response.note_id = note_id;
            response.success = true;
        }
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    Json(response).into_response()
}
