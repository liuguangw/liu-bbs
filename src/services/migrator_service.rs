use crate::common::{Migration, MigrationError, MigrationLog};
use crate::data::MigratorRepository;
use std::time::SystemTime;

/// 数据迁移服务
pub struct MigratorService {
    migrator_repo: MigratorRepository,
}

impl<T: Into<MigratorRepository>> From<T> for MigratorService {
    fn from(item: T) -> Self {
        Self {
            migrator_repo: item.into(),
        }
    }
}

impl MigratorService {
    ///执行数据迁移
    ///
    /// 返回成功迁移的条数
    pub async fn run_migrate(&self, step: Option<u32>) -> Result<u32, MigrationError> {
        let mut action_count = 0;
        let migration_logs = self.migrator_repo.get_migration_logs(1).await?;
        //获取最后一条记录
        let last_log = migration_logs.last();
        let (mut last_log_id, last_batch) = match last_log {
            None => (0, 0),
            Some(s) => (s.id, s.batch),
        };
        let all_migrations = self.migrator_repo.all_migrations();
        for migration_info in &all_migrations {
            //迁移名称
            let migration_name = migration_info.name();
            //判断迁移是否执行过
            let mut migration_log_found = false;
            for migration_log in &migration_logs {
                if migration_log.name == migration_name {
                    migration_log_found = true;
                    break;
                }
            }
            //已经执行过了
            if migration_log_found {
                continue;
            }
            //执行迁移
            println!("migrate {}", migration_name);
            migration_info.up().await?;
            last_log_id += 1;
            //add log
            let migration_log = MigrationLog {
                id: last_log_id,
                name: migration_name.to_string(),
                batch: last_batch + 1,
                created_at: SystemTime::now(),
            };
            self.migrator_repo.insert_log(&migration_log).await?;
            action_count += 1;
            //step限制
            if let Some(step_limit) = step {
                if action_count >= step_limit {
                    break;
                }
            }
        }
        Ok(action_count)
    }
    ///执行数据回滚
    ///
    /// 返回成功回滚的条数
    pub async fn run_rollback(&self, step: Option<u32>) -> Result<u32, MigrationError> {
        let mut action_count = 0;
        //倒序
        let migration_logs = self.migrator_repo.get_migration_logs(-1).await?;
        //获取最后一条记录
        let last_log = migration_logs.first();
        let last_batch = match last_log {
            //没有迁移记录,直接返回
            None => return Ok(0),
            Some(s) => s.batch,
        };
        let all_migrations = self.migrator_repo.all_migrations();
        for migration_log in &migration_logs {
            //判断批次
            if step.is_none() && migration_log.batch != last_batch {
                break;
            }
            //迁移名称
            let migration_name = migration_log.name.as_str();
            let mut migration_info_op: Option<&Box<dyn Migration>> = None;
            for migration_info in &all_migrations {
                if migration_info.name() == migration_name {
                    migration_info_op = Some(migration_info);
                    break;
                }
            }
            //执行回滚
            println!("rollback {}", migration_name);
            if let Some(migration_info) = migration_info_op {
                migration_info.down().await?;
            } else {
                return Err(MigrationError::Common(format!(
                    "migration with name {} not found",
                    migration_name
                )));
            }
            //remove log
            self.migrator_repo.remove_log(migration_log.id).await?;
            action_count += 1;
            //step限制
            if let Some(step_limit) = step {
                if action_count >= step_limit {
                    break;
                }
            }
        }
        Ok(action_count)
    }
}
