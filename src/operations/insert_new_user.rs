// use sqlx::{sqlite::SqliteQueryResult, Error, SqliteConnection};
// use crate::types::NewUser;

// pub async fn insert_new_user(prepared_new_user_data: &NewUser, 
//     database_connection: &mut SqliteConnection)
// -> Result<SqliteQueryResult, Error> {
//     let users_table_insert = sqlx
//         ::query!(
//             "INSERT INTO users (username) VALUES (?)",
//             prepared_new_user_data.username)
//         .execute(database_connection).await;


//     let users_details_table_insert = sqlx
//         ::query!(
//             "INSERT INTO user_details (phonetic_username) VALUES (?)",
//             prepared_new_user_data.phonetic_username
//         )
//         .execute(database_connection).await;

//     let users_credentials_table_insert = sqlx
//         ::query!(
//             "INSERT INTO user_credentials (email, passphrase) VALUES (?, ?)",
//             prepared_new_user_data.email,
//             prepared_new_user_data.passphrase
//         )
//         .execute(database_connection).await;


// }

