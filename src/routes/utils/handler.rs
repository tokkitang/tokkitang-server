use std::sync::Arc;

use aws_sdk_dynamodb::Client;
use aws_sdk_s3::model::ObjectCannedAcl;
use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    models::{InsertUser, User},
    routes::{auth::dto::GithubAccessTokenResponse, user::UserService},
    utils::{generate_uuid, hash_password},
};

use super::{dto::UploadImageResponse, UtilService};

pub async fn router() -> Router {
    Router::new()
        .route("/image/upload/user-thumbnail", post(upload_user_thumbnail))
        .route("/image/upload/team-thumbnail", post(upload_team_thumbnail))
        .route(
            "/image/upload/project-thumbnail",
            post(upload_project_thumbnail),
        )
}

async fn upload_user_thumbnail(
    Extension(s3_client): Extension<Arc<aws_sdk_s3::Client>>,
    mut files: Multipart,
) -> impl IntoResponse {
    let bucket_name = "tokkitang";
    let bucket_url = "https://static.tokkitang.com";

    let _util_service = UtilService::new();

    let mut response = UploadImageResponse {
        image_url: "".into(),
        success: false,
    };

    if let Ok(Some(file)) = files.next_field().await {
        let category = file.name().unwrap().to_string();
        let name = file.file_name().unwrap().to_string();
        let data = file.bytes().await.unwrap();

        let key = format!(
            "thumbnail/user/{}_{}_{}",
            epoch_timestamp::Epoch::now(),
            &category,
            &name
        );

        let image_url = format!("{}/{}", bucket_url, &key);

        let _response = s3_client
            .put_object()
            .bucket(bucket_name)
            .key(&key)
            .body(data.into())
            .set_acl(Some(ObjectCannedAcl::PublicRead))
            .send()
            .await
            .unwrap();

        response.success = true;
        response.image_url = image_url;
    }

    Json(response).into_response()
}

async fn upload_team_thumbnail(
    Extension(s3_client): Extension<Arc<aws_sdk_s3::Client>>,
    mut files: Multipart,
) -> impl IntoResponse {
    let bucket_name = "tokkitang";
    let bucket_url = "https://static.tokkitang.com";

    let _util_service = UtilService::new();

    let mut response = UploadImageResponse {
        image_url: "".into(),
        success: false,
    };

    if let Ok(Some(file)) = files.next_field().await {
        let category = file.name().unwrap().to_string();
        let name = file.file_name().unwrap().to_string();
        let data = file.bytes().await.unwrap();

        let key = format!(
            "thumbnail/team/{}_{}_{}",
            epoch_timestamp::Epoch::now(),
            &category,
            &name
        );

        let image_url = format!("{}/{}", bucket_url, &key);

        let _response = s3_client
            .put_object()
            .bucket(bucket_name)
            .key(&key)
            .body(data.into())
            .set_acl(Some(ObjectCannedAcl::PublicRead))
            .send()
            .await
            .unwrap();

        response.success = true;
        response.image_url = image_url;
    }

    Json(response).into_response()
}

async fn upload_project_thumbnail(
    Extension(s3_client): Extension<Arc<aws_sdk_s3::Client>>,
    mut files: Multipart,
) -> impl IntoResponse {
    let bucket_name = "tokkitang";
    let bucket_url = "https://static.tokkitang.com";

    let _util_service = UtilService::new();

    let mut response = UploadImageResponse {
        image_url: "".into(),
        success: false,
    };

    if let Ok(Some(file)) = files.next_field().await {
        let category = file.name().unwrap().to_string();
        let name = file.file_name().unwrap().to_string();
        let data = file.bytes().await.unwrap();

        let key = format!(
            "thumbnail/project/{}_{}_{}",
            epoch_timestamp::Epoch::now(),
            &category,
            &name
        );

        let image_url = format!("{}/{}", bucket_url, &key);

        let _response = s3_client
            .put_object()
            .bucket(bucket_name)
            .key(&key)
            .body(data.into())
            .set_acl(Some(ObjectCannedAcl::PublicRead))
            .send()
            .await
            .unwrap();

        response.success = true;
        response.image_url = image_url;
    }

    Json(response).into_response()
}
