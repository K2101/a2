use super::Result;
use crate::data;
use crate::data::key_db::Cache;
use crate::domain;
use redis::aio::MultiplexedConnection;
use uuid::Uuid;

pub async fn insert_session_to_cache(
    cache: &Cache,
    web_session: domain::session::WebSession,
) -> Result<()> {
    let (
        user_id,
        email,
        session_id,
        created_at,
        valid_until,
        is_active,
        device_id,
        device_type,
        app_name,
        ip_address,
        location,
        last_active,
    ) = web_session.into_inner();

    let user_id_string = user_id.to_string();

    let created_at = created_at.to_string();
    let valid_until = valid_until.to_string();
    let is_active = is_active.to_string();
    let last_active = last_active.to_string();

    let web_session_list = data::model_key_db::WebSessionRef {
        id: user_id_string.as_str(),
        session_id_list: session_id.as_str(),
    };

    let mut conn = cache.get_conn();
    // EXPIREMEMBER user1 key1 10
    // HMSET
    // HGET
    // lrange 0 -1
    // sadd
    // SMEMBERS
    // SISMEMBER
    // SMISMEMBER multi is member

    // will using ts later ??
    // or use producer ??
    // or when refresh token is called we also query db anyway ??

    redis::pipe()
        // .sadd(web_session_list.id, web_session_list.session_id_list)
        .cmd("SADD")
        .arg(web_session_list.id)
        .arg(web_session_list.session_id_list)
        // not list but single string session_id
        // // .rpush::<&str, &str>(web_session_list.id, web_session_list.session_id_list)
        .ignore()
        .hset_multiple::<&str, &str, &str>(
            session_id.as_str(),
            &[
                ("id", user_id_string.as_str()),
                ("email", email.as_str()),
                ("created_at", created_at.as_str()),
                ("valid_until", valid_until.as_str()),
                ("is_active", is_active.as_str()),
                ("device_id", device_id.as_str()),
                ("device_type", device_type.as_str()),
                ("app_name", app_name.as_str()),
                ("ip_address", ip_address.as_str()),
                ("location", location.as_str()),
                ("last_active", last_active.as_str()),
            ],
        )
        .ignore()
        .cmd("EXPIREMEMBER")
        .arg(session_id.as_str())
        .arg("is_active")
        // 30 sec or 10 minutes
        .arg(60 * 10)
        .ignore()
        .query_async::<MultiplexedConnection, _>(&mut conn)
        .await?;

    Ok(())
}

pub async fn get_session(
    cache: &Cache,
    session_id: &str,
) -> Result<(Option<String>, Option<String>, Option<String>)> {
    let mut conn = cache.get_conn();

    let (id, email, valid_until) = redis::cmd("HMGET")
        .arg(session_id)
        .arg("id")
        .arg("email")
        .arg("valid_until")
        .query_async::<MultiplexedConnection, (Option<String>, Option<String>, Option<String>)>(
            &mut conn,
        )
        .await?;

    Ok((id, email, valid_until))
}

pub async fn update_refresh_session(
    cache: &Cache,
    id: Uuid,
    session_id: &str,
    ip_address: &str,
    location: &str,
    last_active: i64,
) -> Result<()> {
    let mut conn = cache.get_conn();
    let id = id.to_string();
    let is_active = true.to_string();
    let last_active = last_active.to_string();

    // transaction
    redis::pipe()
        .cmd("MULTI")
        .cmd("HMSET")
        .arg(session_id)
        .arg("is_active")
        .arg(is_active)
        .arg("ip_address")
        .arg(ip_address)
        .arg("location")
        .arg(location)
        .arg("last_active")
        .arg(last_active.as_str())
        // 10 minutes
        .cmd("EXPIREMEMBER")
        .arg(session_id)
        .arg("is_active")
        .arg(60 * 10)
        // blind insert to session_id ref to set
        .cmd("SADD")
        .arg(id.as_str())
        .arg(session_id)
        .cmd("EXEC")
        .query_async::<MultiplexedConnection, _>(&mut conn)
        .await?;

    Ok(())
}
