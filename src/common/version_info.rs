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
    ///编译环境的系统信息
    pub compiler_host_os: String,
    ///git提交信息
    pub commit_info: Option<CommitInfo>,
    ///cargo版本
    pub cargo_version: Option<String>,
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
        let cargo_version = option_env!("LIU_BBS_CARGO_VERSION").map(|s| s.to_string());
        VersionInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            compiler_host_os: env!("LIU_BBS_COMPILER_HOST_OS").to_string(),
            commit_info,
            cargo_version,
        }
    }
}
///获取格式化的版本信息字符串
pub fn get_version_string(is_verbose: bool) -> String {
    let version_info = VersionInfo::default();
    let mut version_string = format!("{} {}", env!("CARGO_PKG_NAME"), version_info);
    if is_verbose {
        write!(version_string, "\nversion: {}", version_info.version).unwrap();
        write!(
            version_string,
            "\nbuild-type: {}",
            env!("LIU_BBS_BUILD_PROFILE")
        )
        .unwrap();
        //for GitHub Action
        if option_env!("GITHUB_REPOSITORY").is_some() {
            write!(version_string, "\nbuild-from: GitHub Action").unwrap();
        }
        if let Some(ref commit_info) = version_info.commit_info {
            write!(version_string, "\ncommit-hash: {}", commit_info.commit_hash).unwrap();
            write!(version_string, "\ncommit-date: {}", commit_info.commit_date).unwrap();
        }
        //cargo version
        if let Some(v) = version_info.cargo_version {
            write!(version_string, "\ncargo-version: {}", v).unwrap();
        }
        write!(version_string, "\nhost: {}", env!("LIU_BBS_HOST_TARGET")).unwrap();
        write!(
            version_string,
            "\ncompiler-os: {}",
            version_info.compiler_host_os
        )
        .unwrap();
        //当前运行的环境
        write!(version_string, "\nos: {}", os_info::get()).unwrap();
    }
    version_string
}
