use actix_files::NamedFile;
use actix_web::get;
use std::io;

///openapi文件
#[get("/openapi.yml")]
pub async fn yml_file() -> io::Result<NamedFile> {
    NamedFile::open_async("./openapi.yml").await
}
