pub mod cron_model;

use chrono::{Local, Utc};
use cron::Schedule;
use cron_model::Cron;
use futures::lock::Mutex;
use log::info;
use once_cell::sync::Lazy;
use std::{
    str::FromStr,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use tokio::join;

use crate::api::monitors::monitors_repository;

static CRON_JOBS: Lazy<Arc<Mutex<Vec<Cron>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

pub async fn initialize() {
    info!("Initializing cron jobs...");

    fetch_and_update_cron_jobs().await;
}

async fn fetch_and_update_cron_jobs() {
    loop {
        info!("Fetching cron jobs...");
        let mut cron_jobs = CRON_JOBS.lock().await.clone();
        let monitors_result = monitors_repository::get_all().await;

        cron_jobs.clear();

        match monitors_result {
            Ok(monitors) => {
                for monitor in monitors {
                    cron_jobs.push(Cron::from(monitor));
                }
            }
            Err(_) => {
                info!("Error fetching cron jobs.");
            }
        }

        *CRON_JOBS.lock().await = cron_jobs;
        thread::sleep(Duration::from_secs(60)); // Check database every minute
    }
}

pub async fn run_cron_jobs() {
    // Thread to manage cron job execution
    let mut schedulers: Vec<(cron::Schedule, i32)>; // Store (schedule, job_id) pairs
    thread::sleep(Duration::from_secs(10));

    loop {
        // Update schedulers based on active cron jobs
        {
            let jobs = CRON_JOBS.lock().await.clone();
            schedulers = jobs
                .iter()
                .map(|job| {
                    (
                        Schedule::from_str(job.cron_expression.as_str()).unwrap(),
                        job.id,
                    )
                })
                .collect();
        }

        if CRON_JOBS.lock().await.is_empty() {
            info!("No active cron jobs found.");
            thread::sleep(Duration::from_secs(60));
            continue;
        }

        let tolerance = Duration::from_secs(1);

        let mut next_tick_all = Local::now() + Duration::from_secs(3600);

        // Check for next job execution
        for (schedule, job_id) in schedulers.into_iter() {
            let mut iter = schedule.upcoming(Utc); // Create a new iterator each loop to account for updated schedule
            if let Some(next_tick) = iter.next() {
                let next_tick_local = next_tick.with_timezone(&Local);
                let now_local = Local::now();

                if next_tick_local < next_tick_all {
                    next_tick_all = next_tick_local;
                }

                let jobs = CRON_JOBS.lock().await.clone();

                if let Some(job) = jobs.iter().find(|job| job.id == job_id) {
                    if (next_tick_local - now_local).to_std().unwrap() < tolerance
                        && (job.last_run_time.is_none()
                            || now_local - job.last_run_time.unwrap()
                                > chrono::Duration::from_std(Duration::from_secs(60)).unwrap())
                    {
                        let _ = join!(tokio::spawn(async move {
                            let mut jobs = CRON_JOBS.lock().await.clone();

                            if let Some(job) = jobs.iter().find(|job| job.id == job_id) {
                                info!("Running Job: {:?}", job);

                                monitors_repository::update_last_run_time(job._id.as_str())
                                    .await
                                    .unwrap();

                                if job.cron_type == "https" {
                                    https_monitor(job).await;
                                }

                                // Update last run time
                                jobs.iter_mut().for_each(|job| {
                                    if job.id == job_id {
                                        job.last_run_time = Some(Local::now());
                                    }
                                });
                            }

                            *CRON_JOBS.lock().await = jobs;
                        }));
                    }
                }
            }
        }

        thread::sleep(Duration::from_secs(10));
    }
}

async fn https_monitor(job: &Cron) {
    let client = reqwest::Client::new();
    let start_time = Instant::now();
    let response = client.get(job.url.as_ref().unwrap()).send().await.unwrap();
    let elapsed_time = start_time.elapsed();

    monitors_repository::update_https_monitor_status(
        job._id.as_str(),
        &response.status(),
        &elapsed_time,
    )
    .await
    .unwrap();
}
