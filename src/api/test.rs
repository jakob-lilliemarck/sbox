use actix_web::{get, HttpResponse};

#[get("/test")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body(
        "<a href=\"https://dev-u2n9dnr8.us.auth0.com/authorize?
      response_type=code&
      client_id=Pf4qP0bQup7TRGwl4VPYlikcf0oMWWdf&
      redirect_uri=https://tradr.se&
      scope=appointments%20contacts&
      audience=appointments:api&
      state=xyzABC123\">
      Sign In
    </a>
",
    )
}
