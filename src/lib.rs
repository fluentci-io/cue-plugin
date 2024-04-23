use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "install", &format!("cue@{}", version)])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn fmt(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cue", "fmt", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn export(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cue", "export", &args])?
        .stdout()?;
    Ok(stdout)
}
