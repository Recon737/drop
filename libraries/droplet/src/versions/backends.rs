#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    fs::{metadata, read_dir},
    io::SeekFrom,
    path::{Path, PathBuf},
    process::Stdio,
    sync::LazyLock,
};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::{
    fs::File,
    io::{AsyncReadExt as _, AsyncSeekExt as _, BufReader},
    process::Command,
};

use crate::versions::types::{MinimumFileObject, VersionBackend, VersionFile};

pub fn _list_files(vec: &mut Vec<PathBuf>, path: &Path) -> Result<()> {
    if metadata(path)?.is_dir() {
        let paths = read_dir(path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            if metadata(&full_path)?.is_dir() {
                _list_files(vec, &full_path)?;
            } else {
                vec.push(full_path);
            }
        }
    };

    Ok(())
}

#[derive(Clone)]
pub struct PathVersionBackend {
    pub base_dir: PathBuf,
}

#[async_trait]
impl VersionBackend for PathVersionBackend {
    async fn list_files(&mut self) -> anyhow::Result<Vec<VersionFile>> {
        let mut vec = Vec::new();
        _list_files(&mut vec, &self.base_dir)?;

        let mut results = Vec::new();

        for pathbuf in vec.iter() {
            let relative = pathbuf.strip_prefix(self.base_dir.clone())?;

            results.push(
                self.peek_file(
                    relative
                        .to_str()
                        .ok_or(anyhow!("Could not parse path"))?
                        .to_owned(),
                )
                .await?,
            );
        }

        Ok(results)
    }

    async fn reader(
        &mut self,
        file: &VersionFile,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Box<dyn MinimumFileObject>> {
        let mut file = File::open(self.base_dir.join(file.relative_filename.clone())).await?;

        if start != 0 {
            file.seek(SeekFrom::Start(start)).await?;
        }

        if end != 0 {
            return Ok(Box::new(file.take(end - start)));
        }

        Ok(Box::new(file))
    }

    async fn peek_file(&mut self, sub_path: String) -> anyhow::Result<VersionFile> {
        let pathbuf = self.base_dir.join(sub_path.clone());
        if !pathbuf.exists() {
            return Err(anyhow!("Path doesn't exist."));
        };

        let file = File::open(pathbuf.clone()).await?;
        let metadata = file.try_clone().await?.metadata().await?;
        let permission_object = metadata.permissions();
        let permissions = {
            let perm: u32;
            #[cfg(target_family = "unix")]
            {
                perm = permission_object.mode();
            }
            #[cfg(not(target_family = "unix"))]
            {
                perm = 0
            }
            perm
        };

        Ok(VersionFile {
            relative_filename: sub_path,
            permission: permissions,
            size: metadata.len(),
        })
    }

    fn require_whole_files(&self) -> bool {
        false
    }
}

pub static SEVEN_ZIP_INSTALLED: LazyLock<bool> =
    LazyLock::new(|| std::process::Command::new("7z").output().is_ok());
// https://7-zip.opensource.jp/chm/general/formats.htm
// Intentionally repeated some because it's a trivial cost and it's easier to directly copy from the docs above
pub const SUPPORTED_FILE_EXTENSIONS: [&str; 89] = [
    "7z", "bz2", "bzip2", "tbz2", "tbz", "gz", "gzip", "tgz", "tar", "wim", "swm", "esd", "xz",
    "txz", "zip", "zipx", "jar", "xpi", "odt", "ods", "docx", "xlsx", "epub", "apm", "ar", "a",
    "deb", "lib", "arj", "cab", "chm", "chw", "chi", "chq", "msi", "msp", "doc", "xls", "ppt",
    "cpio", "cramfs", "dmg", "ext", "ext2", "ext3", "ext4", "img", "fat", "img", "hfs", "hfsx",
    "hxs", "hxr", "hxq", "hxw", "lit", "ihex", "iso", "img", "lzh", "lha", "lzma", "mbr", "mslz",
    "mub", "nsis", "ntfs", "img", "mbr", "rar", "r00", "rpm", "ppmd", "qcow", "qcow2", "qcow2c",
    "squashfs", "udf", "iso", "img", "scap", "uefif", "vdi", "vhd", "vmdk", "xar", "pkg", "z",
    "taz",
];

#[derive(Clone)]
pub struct ZipVersionBackend {
    path: String,
}
impl ZipVersionBackend {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            path: path.to_str().expect("invalid utf path").to_owned(),
        })
    }
}

#[async_trait]
impl VersionBackend for ZipVersionBackend {
    async fn list_files(&mut self) -> anyhow::Result<Vec<VersionFile>> {
        let mut list_command = Command::new("7z");
        list_command.args(vec!["l", "-ba", &self.path]);
        let result = list_command.output().await?;
        if !result.status.success() {
            return Err(anyhow!(
                "failed to list files: code {:?}",
                result.status.code()
            ));
        }
        let raw_result = String::from_utf8(result.stdout)?;
        let files = raw_result
            .split("\n")
            .filter(|v| v.len() > 0)
            .map(|v| v.split(" ").filter(|v| v.len() > 0));
        let mut results = Vec::new();

        for file in files {
            let values = file.collect::<Vec<&str>>();
            let mut iter = values.iter();
            let (date, time, attrs, size, compress, name) = (
                iter.next().expect("failed to read date"),
                iter.next().expect("failed to read time"),
                iter.next().expect("failed to read attrs"),
                iter.next().expect("failed to read size"),
                iter.next().expect("failed to read compress"),
                iter.collect::<Vec<&&str>>(),
            );
            if attrs.starts_with("D") {
                continue;
            }
            results.push(VersionFile {
                relative_filename: name
                    .into_iter()
                    .map(|v| *v)
                    .fold(String::new(), |a, b| a + b + " ")
                    .trim_end()
                    .to_owned(),
                permission: 0o744, // owner r/w/x, everyone else, read
                size: size.parse().unwrap(),
            });
        }

        Ok(results)
    }

    async fn reader(
        &mut self,
        file: &VersionFile,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Box<dyn MinimumFileObject>> {
        let mut read_command = Command::new("7z");
        read_command.args(vec!["e", "-so", &self.path, &file.relative_filename]);
        let mut output = read_command
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn 7z");
        let stdout = output
            .stdout
            .take()
            .ok_or(anyhow!("no stdout on 7zip process"))?;
        let reader = BufReader::new(stdout);
        Ok(Box::new(reader))
    }

    async fn peek_file(&mut self, sub_path: String) -> anyhow::Result<VersionFile> {
        let files = self.list_files().await?;
        let file = files
            .iter()
            .find(|v| v.relative_filename == sub_path)
            .expect("file not found");

        Ok(file.clone())
    }

    fn require_whole_files(&self) -> bool {
        true
    }
}
