use std::process::{Command, Stdio};
use std::env;

fn main() {
    let user = "postgres".to_string();
    let db_name = "dvdrental";
    let backup_path = "/docker-entrypoint-initdb.d/dvdrental.tar";

    println!("e7e0d652 Dropping database {} if it exists...", db_name);
    let _ = Command::new("psql")
        .args(["-U", &user, "-c", &format!("DROP DATABASE IF EXISTS \"{}\";", db_name)])
        .status();

    println!("e7e0d652 Creating database {}...", db_name);
    let create_status = Command::new("psql")
        .args(["-U", &user, "-c", &format!("CREATE DATABASE \"{}\";", db_name)])
        .status()
        .expect("Failed to execute psql create");

    if create_status.success() {
        println!("e7e0d652 Restoring backup from {}...", backup_path);
        let restore_status = Command::new("pg_restore")
            .args(["-U", &user, "-d", db_name, backup_path])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Failed to execute pg_restore");

        if restore_status.success() {
            println!("e7e0d652 Successfully restored {}!", db_name);
        } else {
            println!("e7e0d652 Restoration failed {}!", db_name);
            std::process::exit(1);
        }
    }
}