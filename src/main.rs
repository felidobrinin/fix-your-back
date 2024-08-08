use anyhow::Result;
use notify_rust::Notification;
use rand::{distributions::Uniform, prelude::Distribution};
use serde::Deserialize;
use std::{fs, thread, time::Duration};

#[derive(Debug, Deserialize)]
struct Config {
    // Wait minimum ammout of minutes between batch of notifications
    wait_min: u64,
    // Wait maximum ammout of minutes between batch of notifications
    wait_max: u64,
    // Ammount of notifications sent on a batch
    notifications: u64,
    // Makes the notifications critical
    critical: bool,
    // Notification lifespan in seconds
    lifespan: u64,
}

fn main() -> Result<()> {
    println!("Arregla La Espalda STARTED");
    println!("Trying to read config file");

    // Config file
    let cfg = read_config("/etc/fix-your-back/cfg.toml")?;
    println!("{:?}", cfg);

    // Main Loop
    loop {
        // Generate time for sleep
        let mut rng = rand::thread_rng();
        let mins = Uniform::from(cfg.wait_min..=cfg.wait_max).sample(&mut rng);
        let seg = mins * 60;
        println!("Esperamos {} minutos, o {} segundos", mins, seg);

        // Sleep thread
        thread::sleep(Duration::from_secs(seg));
        println!("Pasó la espera");

        // Print notifications
        for _ in 1..=cfg.notifications {
            Notification::new()
                .summary("FIX YOUR POSTURE")
                .body("FIX YOUR POSTURE OR LOSE YOUR BACK AT YOUR 20s")
                .icon("warning-outline-symbolic")
                .appname("YOUR BACK IN PAIN")
                .urgency(match cfg.critical {
                    true => notify_rust::Urgency::Critical,
                    false => notify_rust::Urgency::Normal,
                })
                .timeout(Duration::from_secs(10)) // 5 seconds of wait before closing the notifications automatically
                .show()
                .unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn read_config(cfg_path: &str) -> Result<Config> {
    let cfg_content = fs::read_to_string(cfg_path)?;
    let cfg: Config = toml::from_str(&cfg_content)?;

    Ok(Config {
        wait_min: cfg.wait_min.clamp(1, 60 * 24),
        wait_max: cfg.wait_max.clamp(1, 60 * 24),
        notifications: cfg.notifications.clamp(1, 10),
        lifespan: cfg.lifespan.clamp(3, 60 * 24),
        critical: cfg.critical,
    })
}
