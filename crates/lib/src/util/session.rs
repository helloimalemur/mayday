// use sea_orm::DatabaseConnection;
// use crate::session::SessionId;
//
// pub async fn delete_session_by_userid(user_id: i16, db_pool: DatabaseConnection) {
//     let userid = user_id as i32;
//     if let Ok(_query_result) = sqlx::query("DELETE FROM session WHERE userid=(?)")
//         .bind(userid.clone())
//         .execute(db_pool.get_mysql_connection_pool())
//         .await
//     {
//         println!("SESSION DELETED :: {}", user_id.clone());
//     }
// }
//
// #[allow(unused)]
// pub async fn delete_session_by_email(email: String, db_pool: DatabaseConnection) {
//     if let Ok(_query_result) = sqlx::query("DELETE FROM session WHERE email=(?)")
//         .bind(email.clone())
//         .execute(db_pool.get_mysql_connection_pool())
//         .await
//     {
//         println!("SESSION DELETED :: {}", email.clone());
//     }
// }
//
// #[allow(unused)]
// pub async fn check_if_session_exists_with_user_id(
//     user_id: i16,
//     session_id: SessionId,
//     db_pool: DatabaseConnection,
// ) -> bool {
//     println!("{} - {}", user_id, session_id.session_id);
//     let result = sqlx::query("SELECT (1) FROM session WHERE sessionid=(?) AND userid=(?)")
//         .bind(session_id.session_id)
//         .bind(user_id)
//         .fetch_all(db_pool.get_mysql_connection_pool())
//         .await
//         .unwrap();
//
//     // let queried_sessionid: String = result.get("sessionid").unwrap();
//     // println!("{}", queried_sessionid);
//
//     // let queried_user_id: i16 = result.get("userid").unwrap();
//
//     true
// }
