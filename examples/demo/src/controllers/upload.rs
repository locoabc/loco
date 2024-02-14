use std::path::PathBuf;

use axum::extract::Multipart;
use loco_rs::prelude::*;

use crate::views;

/// File upload example
///
/// ## Request Example
///
/// curl -H "Content-Type: multipart/form-data" -F "file=@./test-2.json"
/// 127.0.0.1:3000/upload/file
async fn upload_file(
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Json<views::upload::Response>> {
    let mut file = None;
    while let Some(field) = multipart.next_field().await.map_err(|err| {
        tracing::error!(error = ?err,"could not readd multipart");
        Error::BadRequest("could not readd multipart".into())
    })? {
        let file_name = match field.file_name() {
            Some(file_name) => file_name.to_string(),
            _ => return Err(Error::BadRequest("file name not found".into())),
        };

        let content = field.bytes().await.map_err(|err| {
            tracing::error!(error = ?err,"could not readd bytes");
            Error::BadRequest("could not readd bytes".into())
        })?;

        let path = PathBuf::from("folder").join(file_name);
        ctx.storage
            .as_ref()
            .unwrap()
            .upload(path.as_path(), &content)
            .await?;

        file = Some(path);
    }

    file.map_or_else(not_found, |path| {
        format::json(views::upload::Response::new(path.as_path()))
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("upload")
        .add("/file", post(upload_file))
}
