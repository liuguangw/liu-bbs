use super::commands::Command;
use crate::common::{AppConfig, DatabaseData, MigrationError};
use crate::rt;
use crate::services::MigratorService;
use clap::Args;
use std::sync::Arc;

/// 数据迁移命令
#[derive(Args)]
pub struct MigrateCommand {
    ///step limit
    #[arg(short, long, value_name = "STEP_LIMIT")]
    step: Option<u32>,
    ///rollback data
    #[arg(long)]
    rollback: bool,
    ///Path of configuration file
    #[arg(short = 'C', long = "conf", default_value_t = String::from("./config.toml"), value_name = "FILE")]
    config_file_path: String,
}

impl MigrateCommand {
    async fn launch(&self) -> Result<(), MigrationError> {
        let app_config = AppConfig::load(&self.config_file_path).await?;
        let database_data = DatabaseData::connect(&app_config.database).await?;
        let database_data = Arc::new(database_data);
        let migrator_service = MigratorService::from(&database_data);
        if self.rollback {
            self.run_rollback(&migrator_service, self.step).await?;
        } else {
            self.run_migrate(&migrator_service, self.step).await?;
        }
        Ok(())
    }
    ///迁移
    async fn run_migrate(
        &self,
        migrator_service: &MigratorService,
        step: Option<u32>,
    ) -> Result<(), MigrationError> {
        let migrate_count = migrator_service.run_migrate(step).await?;
        if migrate_count == 0 {
            println!("Nothing to migrate");
        } else {
            println!("Migrate count: {}", migrate_count);
        }
        Ok(())
    }
    ///回滚
    async fn run_rollback(
        &self,
        migrator_service: &MigratorService,
        step: Option<u32>,
    ) -> Result<(), MigrationError> {
        let migrate_count = migrator_service.run_rollback(step).await?;
        if migrate_count == 0 {
            println!("Nothing to rollback");
        } else {
            println!("Rollback count: {}", migrate_count);
        }
        Ok(())
    }
}

impl Command for MigrateCommand {
    fn execute(&self) {
        if let Err(err) = rt::block_on(self.launch()) {
            panic!("{err}");
        }
    }
}
