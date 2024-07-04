use futures::TryStreamExt;

use crate::{api::auth::auth_enum::MontingerError, db};

use super::monitors_model::Monitor;

pub async fn get_all() -> Result<Vec<Monitor>, MontingerError> {
    let client_arc = db::get_client().await?;

    let client = client_arc
        .lock()
        .map_err(|_| MontingerError::PoisonedMutex)?
        .clone();

    let database = client.database("montinger");
    let collection = database.collection::<Monitor>("monitors");

    let cursor = collection.find(None, None).await?;
    let monitors = cursor.try_collect::<Vec<Monitor>>().await?;

    Ok(monitors)
}
