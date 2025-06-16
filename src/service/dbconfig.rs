use crate::utils::{app_error::AppError, web_error::WebError};
use config::Config;
use migration::MigratorTrait;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

pub static CONNECT_POOL: OnceCell<DatabaseConnection> = OnceCell::new();

#[derive(Debug, Deserialize, Serialize)]
pub struct DbSetting {
    database_config: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    dbname: String,
    username: String,
    password: String,
    max_connections: u32,
    min_connections: u32,
}

impl DatabaseConfig {
    /**
     * 初始化链接
     */
    pub async fn init_database() -> Result<(), AppError> {
        info!("开始初始化数据库！！");
        let setting: DbSetting = Config::builder()
            .add_source(config::File::with_name("src/config/config.toml"))
            .build()
            .unwrap()
            .try_deserialize()
            .map_err(|e| {
                let err_msg = format!("数据库配置解析失败: {}", e);
                error!("{}", err_msg);
                AppError::ConfigError(err_msg)
            })?;

        let config = setting.database_config;
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.dbname
        );
        info!("准备数据库连接信息{:?}", database_url);
        let mut connect_option = ConnectOptions::new(database_url);

        connect_option
            .max_connections(config.max_connections)
            .min_connections(config.min_connections);

        let connect = Database::connect(connect_option).await.map_err(|e| {
                let err_msg = format!("数据库连接失败: {}", e);
                error!("{}", err_msg);
                AppError::Database(e)
            })?;
        migration::Migrator::up(&connect, None).await?;
        CONNECT_POOL.get_or_init(move || connect);

        Ok(())
    }

    /**
     * 获取数据库连接
     */
    pub async fn get_connect() -> Result<DatabaseConnection, WebError> {
        if let Some(connect) = CONNECT_POOL.get() {
            info!("拿到数据库连接！！");
            Ok(connect.clone())
        } else {
            Err(WebError::DBError(
                "Database connection pool not initialized".into(),
            ))
        }
    }
}
