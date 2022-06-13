use std::ffi::OsString;
use std::path::Path;



///
/// 返回当前 workspace 下 同 cargo.toml 文件 package 路径中文件
/// path -> join ./cargo.toml/../{path}
///
pub fn path_resolve(path: &str) -> String {
  let work_cwd = env!("CARGO_MANIFEST_DIR");
  let os_work_cwd = OsString::from(work_cwd);
  Path::new(&os_work_cwd)
    .join(path)
    .into_os_string()
    .into_string()
    .unwrap()
}

///
/// 执行安全的 读取 某路径文件
///
pub fn readfile(path: &str) -> Result<String, String> {
  let filepath = Path::new(path);
  
  if filepath.exists() {
    if filepath.is_dir() {
      return Err(format!(
        "file is not file maybe is dir ?! filepath is{}",
        path
      ));
    }
    match std::fs::read_to_string(filepath) {
      Ok(content) => Ok(content),
      Err(ex) => Err(ex.to_string()),
    }
  } else {
    Err(format!("file is not exists filepath is {}", path))
  }
}
