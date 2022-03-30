
use tera::{Tera, Context};
use actix_web::{web, HttpResponse, Error,  error};

pub async fn home(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    
    let mut ctx = Context::new();
    ctx.insert("name", "kz_morita"); // 変数をctxにプッシュ
    let view = tmpl.render("home.html.tera", &ctx)
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
   
}