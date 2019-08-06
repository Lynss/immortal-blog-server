use crate::utils;
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, web};
use common::{HandlerResponse, ImmortalError};
use futures::future::IntoFuture;
use futures::{Future, Stream};
use std::{env, fs::File, io::Write};

fn save_file(field: Field) -> impl Future<Item = String, Error = ImmortalError> {
    let content_disposition = field.content_disposition().unwrap();
    let file_name = format!(
        "{}_{}",
        utils::uuid(),
        content_disposition.get_filename().unwrap_or("unknown"),
    );
    let file_path_string = format!("static/{}", file_name);
    File::create(&file_path_string)
        .map_err(|_| MultipartError::Incomplete)
        .map(|file| {
            field
                .fold(file, |mut file, bytes| {
                    // fs operations are blocking, we have to execute writes on thread pool
                    web::block(move || {
                        file.write_all(bytes.as_ref())
                            .map_err(|_| MultipartError::Incomplete)?;
                        Ok(file)
                    })
                    .map_err(
                        |e: error::BlockingError<MultipartError>| match e {
                            error::BlockingError::Error(e) => e,
                            error::BlockingError::Canceled => MultipartError::Incomplete,
                        },
                    )
                })
                .map(move |_| {
                    format!(
                        "{}/{}",
                        env::var("BACKEND_SERVER_ADDRESS").unwrap(),
                        file_path_string
                    )
                })
        })
        .into_future()
        .flatten()
        .map_err(ImmortalError::ignore)
}

pub fn upload_file(multipart: Multipart) -> impl HandlerResponse<Vec<String>> {
    multipart
        .map_err(ImmortalError::ignore)
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(utils::success)
}
