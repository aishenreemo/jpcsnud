use clap::Parser;
use clap::Subcommand;
use semver::Version;
use std::fs;
use std::process::exit;
use std::process::Command;

/// CLI for managing local PostgreSQL
#[derive(Parser)]
#[command(name = "pg_cli")]
#[command(author = "Aivan")]
#[command(version = "0.1")]
#[command(about = "Manage project-local PostgreSQL", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CheckHealth,
    Mkdir,
    Start,
    Stop,
    Status,
    Sql,
}

fn main() {
    let cli = Cli::parse();

    let base_dir = std::env::current_dir().expect("Failed to get current dir");
    let backend_dir = base_dir.join("backend");
    let pgdata = backend_dir.join("pgdata");
    let pgsocket = backend_dir.join("pgsocket");
    let pglog = backend_dir.join("pglog");
    let migrations = backend_dir.join("migrations");

    match cli.command {
        Commands::Mkdir => {
            fs::create_dir_all(&pgdata).unwrap();
            fs::create_dir_all(&pgsocket).unwrap();
            fs::create_dir_all(&pglog).unwrap();
            fs::create_dir_all(&migrations).unwrap();
            println!(
                "Directories created:\n  pgdata: {:?}\n  pgsocket: {:?}\n  pglog: {:?}\n migrations: {:?}",
                pgdata, pgsocket, pglog, migrations
            );
        }

        Commands::Start => {
            if !pgdata.join("PG_VERSION").exists() {
                println!("Initializing database...");
                let status = Command::new("initdb")
                    .arg("-D")
                    .arg(&pgdata)
                    .status()
                    .expect("Failed to run initdb");
                if !status.success() {
                    eprintln!("initdb failed");
                    exit(1);
                }
            }

            println!("Starting PostgreSQL...");
            let status = Command::new("pg_ctl")
                .arg("-D")
                .arg(&pgdata)
                .arg("-o")
                .arg(format!("-k {} -h 127.0.0.1", pgsocket.display()))
                .arg("-l")
                .arg(pglog.join("server.log"))
                .arg("start")
                .status()
                .expect("Failed to start PostgreSQL");
            if !status.success() {
                eprintln!("pg_ctl start failed");
                exit(1);
            }
            println!("PostgreSQL started.");
        }

        Commands::Stop => {
            let status = Command::new("pg_ctl")
                .arg("-D")
                .arg(&pgdata)
                .arg("stop")
                .status()
                .expect("Failed to stop PostgreSQL");
            if !status.success() {
                eprintln!("pg_ctl stop failed");
                exit(1);
            }
            println!("PostgreSQL stopped.");
        }

        Commands::Status => {
            let status = Command::new("pg_ctl")
                .arg("-D")
                .arg(&pgdata)
                .arg("status")
                .status()
                .expect("Failed to check status");
            exit(status.code().unwrap_or(1));
        }

        Commands::Sql => {
            dotenv::dotenv().ok();
            let database_url = std::env::var("DATABASE_URL")
                .expect("DATABASE_URL not set in .env");

            let username = whoami::username();
            let url = url::Url::parse(&database_url).expect("Invalid DATABASE_URL format");
            let user = url.username();
            let password = url.password().unwrap_or("");
            let dbname = url.path().trim_start_matches('/');
            let host = url.host_str().unwrap_or("127.0.0.1");
            let port = url.port().unwrap_or(5432).to_string();

            let superuser_status = Command::new("psql")
                .arg("-U")
                .arg(username)
                .arg("-d")
                .arg("postgres")
                .arg("-h")
                .arg(host)
                .arg("-p")
                .arg(&port)
                .arg("-c")
                .arg(format!("DO $$ BEGIN
                        IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = '{}') THEN
                        CREATE ROLE {} LOGIN PASSWORD '{}';
                        END IF;
                        IF NOT EXISTS (SELECT FROM pg_database WHERE datname = '{}') THEN
                        CREATE DATABASE {} OWNER {};
                        END IF;
                        END $$;", user, user, password, dbname, dbname, user))
                .status()
                .expect("Failed to ensure database and user exist");

            if !superuser_status.success() {
                eprintln!("Failed to create database or user. Ensure 'postgres' superuser can connect.");
                exit(1);
            }

            let status = Command::new("psql")
                .arg("-U")
                .arg(user)
                .arg("-d")
                .arg(dbname)
                .arg("-h")
                .arg(host)
                .arg("-p")
                .arg(port)
                .status()
                .expect("Failed to run psql");

            if !status.success() {
                eprintln!("psql failed");
                exit(1);
            }
        }

        Commands::CheckHealth => {
            check_version("postgres", "17.0", &["--version"]);
            check_version("sqlx", "0.8.0", &["--version"]);
            check_version("dx", "0.6.0", &["--version"]);
            check_version("tailwindcss", "4.0.0", &["--version"]);

            println!("All required tools are installed and meet minimum versions!");
        }
    }
}

fn check_version(cmd: &str, min_version: &str, args: &[&str]) {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .unwrap_or_else(|_| {
            eprintln!("❌ {} is not installed or not in PATH", cmd);
            std::process::exit(1);
        });

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version_line = stdout.lines().next().unwrap_or("");
    let version_str = version_line.chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect::<String>();

    let version = normalize_version(&version_str);
    let min_version = normalize_version(min_version);

    let version = Version::parse(&version).unwrap_or_else(|_| panic!("❌ Failed to parse version of {}\n{version}", cmd));
    let min_version = Version::parse(&min_version).unwrap_or_else(|_| panic!("❌ Failed to parse version of {}\n{min_version}", cmd));

    if version < min_version {
        eprintln!("❌ {} version {} < required {}", cmd, version, min_version);
        std::process::exit(1);
    }

    println!("✅ {} version {} OK", cmd, version);
}

fn normalize_version(raw: &str) -> String {
    let mut parts: Vec<&str> = raw.split('.').collect();

    while parts.len() < 3 {
        parts.push("0");
    }

    parts.truncate(3);
    parts.join(".")
}
