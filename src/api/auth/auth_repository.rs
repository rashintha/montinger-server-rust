use crate::db;

use super::{auth_enum::MontingerError, auth_model::User};

pub async fn get_user_by_email(email: &str) -> Result<Option<User>, MontingerError> {
    let client_arc = db::get_client().await?;

    let client = client_arc
        .lock()
        .map_err(|_| MontingerError::PoisonedMutex)?
        .clone();

    let database = client.database("montinger");
    let collection = database.collection::<User>("users");

    let filter = mongodb::bson::doc! { "email": email };
    let user = collection.find_one(filter, None).await?;

    Ok(user)
}
