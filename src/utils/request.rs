use actix_web::HttpRequest;

pub fn get_param(name: &str, request: HttpRequest) -> String {
    request.match_info().get(name).unwrap().parse().unwrap()
}