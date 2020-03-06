use actix_multipart::*;
use actix_web::{http::header::LOCATION, post, web::Data, Error, HttpRequest, HttpResponse};
use async_std::fs::File;
use futures::{AsyncWriteExt, StreamExt};

use crate::config::Config;

#[post("/api/upload")]
pub async fn upload(
    req: HttpRequest,
    config: Data<Config>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let has_key = req
        .headers()
        .get("key")
        .map(|val| val.to_str().unwrap().to_string())
        .map(|key| config.keys.contains(&key))
        .unwrap_or(false);

    if !has_key {
        return Ok(HttpResponse::Unauthorized().body("invalid key provided"));
    }

    let config: &Config = config.as_ref();
    let mut name = None;

    while let Some(Ok(field)) = payload.next().await {
        let file_name = upload_field(config, field).await?;
        name = Some(file_name);
    }

    let response = match name {
        Some(name) => HttpResponse::PermanentRedirect()
            .header(LOCATION, config.redirect_template.replace("$FILE", &name))
            .finish(),
        None => HttpResponse::BadRequest().body("no files uploaded"),
    };

    Ok(response)
}

async fn upload_field(config: &Config, mut field: Field) -> Result<String, Error> {
    let name = config.name_generator.generate_name();
    let name = field
        .content_disposition()
        .and_then(|content_disposition| content_disposition.get_filename().map(String::from))
        .and_then(|ext| ext.split(".").last().map(String::from))
        .map(|ext| format!("{}.{}", name, ext))
        .unwrap_or(name);

    let mut file = File::create(config.upload_directory.join(&name)).await?;
    let mut length = 0;

    while let Some(bytes) = field.next().await {
        let bytes = bytes?;
        length += bytes.len();

        if length / 1000 > config.max_file_size {
            return Err(Error::from(
                HttpResponse::PayloadTooLarge().body("payload is too big"),
            ));
        }

        file.write_all(&bytes).await?;
    }

    Ok(name)
}
