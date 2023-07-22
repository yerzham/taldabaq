use wit_parser::{PackageId, Resolve, UnresolvedPackage};
use std::path::{Path, PathBuf};

pub fn parse_source(source: &str) -> anyhow::Result<(Resolve, PackageId, Vec<PathBuf>)> {
  let mut resolve = Resolve::default();
  let mut files = Vec::new();
  let mut parse = |path: &Path| -> anyhow::Result<_> {
      if path.is_dir() {
          let (pkg, sources) = resolve.push_dir(&path)?;
          files = sources;
          Ok(pkg)
      } else {
          let pkg = UnresolvedPackage::parse_file(path)?;
          files.extend(pkg.source_files().map(|s| s.to_owned()));
          resolve.push(pkg)
      }
  };
  let pkg = parse(Path::new(&source))?;

  Ok((resolve, pkg, files))
}