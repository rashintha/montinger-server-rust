use bson::{doc, oid::ObjectId, DateTime};
use futures::TryStreamExt;
use std::str::FromStr;

use crate::{api::auth::auth_enum::MontingerError, db};

use super::monitors_model::{Monitor, MonitorData};

pub async fn get_all() -> Result<Vec<Monitor>, MontingerError> {
    let client_arc = db::get_client().await?;

    let client = client_arc
        .lock()
        .map_err(|_| MontingerError::PoisonedMutex)?
        .clone();

    let database = client.database("montinger");
    let collection = database.collection::<Monitor>("monitors");

    let filter = doc! {"status": {"$ne": "deleted"}};

    let cursor = collection.find(filter, None).await?;
    let monitors = cursor.try_collect::<Vec<Monitor>>().await?;

    Ok(monitors)
}

pub async fn update_last_run_time(monitor_id: &str) -> Result<(), MontingerError> {
    let client_arc = db::get_client().await?;

    let client = client_arc
        .lock()
        .map_err(|_| MontingerError::PoisonedMutex)?
        .clone();

    let database = client.database("montinger");
    let collection = database.collection::<Monitor>("monitors");

    let filter = doc! {"_id": ObjectId::from_str(monitor_id).unwrap()};
    let update = doc! {"$set": {"last_run_time": DateTime::now()}};

    collection.update_one(filter, update, None).await?;

    Ok(())
}

pub async fn update_https_monitor_status(
    monitor_id: &str,
    status: &reqwest::StatusCode,
    response_time: &std::time::Duration,
) -> Result<(), MontingerError> {
    let client_arc = db::get_client().await?;

    let client = client_arc
        .lock()
        .map_err(|_| MontingerError::PoisonedMutex)?
        .clone();

    let database = client.database("montinger");
    let collection = database.collection::<Monitor>("monitors");

    // let filter = doc! {"_id": ObjectId::from_str(monitor_id).unwrap()};
    // let update_options = UpdateOptions::builder().array_filters(None).build();


    let pipeline = vec![
        doc! {
            "$match": {
                "_id": ObjectId::from_str(monitor_id).unwrap()
            }
        },
        doc! {
            "$set": {
                "history_data": {
                    "$concatArrays": [
                        {
                            "$slice": [
                                "$history_data",
                                { "$max": [0, { "$subtract": [{ "$size": "$history_data" }, 19] }] },
                                20
                            ]
                        },
                        [{
                            "time": DateTime::now(),
                            "response_time": response_time.as_millis() as i64,
                            "status_code": status.as_u16() as i64,
                        }]
                    ]
                }
            }
        },
        doc! {
            "$merge": {
                "into": collection.name(),
                "on": "_id",
                "whenMatched": "replace"
            }
        }
    ];

    collection
        .aggregate(pipeline, None)
        .await?;

    add_to_https_history(monitor_id, status, response_time).await?;

    Ok(())
}

pub async fn add_to_https_history(
    monitor_id: &str,
    status: &reqwest::StatusCode,
    response_time: &std::time::Duration,
) -> Result<(), MontingerError> {
    let client_arc = db::get_client().await?;

    let client = client_arc
        .lock()
        .map_err(|_| MontingerError::PoisonedMutex)?
        .clone();

    let database = client.database("montinger");
    let collection = database.collection::<MonitorData>("monitors_history");

    let add = MonitorData {
        id: Some(ObjectId::new()),
        monitor_id: Some(ObjectId::from_str(monitor_id).unwrap()),
        time: DateTime::now(),
        response_time: response_time.as_millis() as i64,
        status_code: status.as_u16() as i64,
    };

    collection.insert_one(add, None).await?;

    Ok(())
}
