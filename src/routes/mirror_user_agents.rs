use headers::HeaderMap;

pub async fn mirror_user_agents(headers: HeaderMap) -> String{
    let message_value = headers.get("User-Agent").unwrap();
    let message = message_value.to_str().unwrap().to_owned();
    message
}
