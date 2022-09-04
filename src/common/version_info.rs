use std::fmt;
use std::fmt::Write;

///git提交信息
pub struct CommitInfo {
    ///提交的短hash
    pub short_commit_hash: String,
    ///提交的完整hash
    pub commit_hash: String,
    ///提交日期
    pub commit_date: String,
}
///版本信息
pub struct VersionInfo {
    ///版本号
    pub version: String,
    ///编译时间
    pub build_time: String,
    ///编译环境的系统信息
    pub compiler_host_os: String,
    ///git提交信息
    pub commit_info: Option<CommitInfo>,
}
impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)?;

        if let Some(ref ci) = self.commit_info {
            write!(f, " ({} {})", ci.short_commit_hash, ci.commit_date)?;
        };
        Ok(())
    }
}

impl Default for VersionInfo {
    fn default() -> Self {
        let commit_info =
            option_env!("LIU_BBS_COMMIT_SHORT_HASH").map(|short_commit_hash| CommitInfo {
                short_commit_hash: short_commit_hash.to_string(),
                commit_hash: env!("LIU_BBS_COMMIT_HASH").to_string(),
                commit_date: env!("LIU_BBS_COMMIT_DATE").to_string(),
            });
        VersionInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_time: env!("LIU_BBS_BUILD_TIME").to_string(),
            compiler_host_os: env!("LIU_BBS_COMPILER_HOST_OS").to_string(),
            commit_info,
        }
    }
}
///获取格式化的版本信息字符串
pub fn get_version_string(is_verbose: bool) -> String {
    let version_info = VersionInfo::default();
    let mut version_string = format!("{} {}", env!("CARGO_PKG_NAME"), version_info);
    if is_verbose {
        write!(version_string, "\nrelease: {}", version_info.version).unwrap();
        if let Some(ref commit_info) = version_info.commit_info {
            write!(version_string, "\ncommit-hash: {}", commit_info.commit_hash).unwrap();
            write!(version_string, "\ncommit-date: {}", commit_info.commit_date).unwrap();
        }
        write!(version_string, "\nbuild-time: {}", version_info.build_time).unwrap();
        if option_env!("GITHUB_REPOSITORY").is_some() {
            write!(version_string, "\nbuild-method: GitHub Action").unwrap();
        } else {
            write!(version_string, "\nbuild-method: common").unwrap();
        }
        write!(
            version_string,
            "\ncompiler-host-os: {}",
            version_info.compiler_host_os
        )
        .unwrap();
        //当前运行的环境
        write!(version_string, "\ncurrent-host-os:  {}", os_info::get()).unwrap();
    }
    version_string
}
