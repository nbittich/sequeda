use std::{error::Error, path::PathBuf, process::Stdio};

use tokio::process::Command;

pub async fn convert_to(
    input_path: impl Into<PathBuf>,
    to: ConvertType,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let input_path: PathBuf = input_path.into();
    tracing::debug!("convert file {input_path:?}");
    let input_path_str = &input_path.display().to_string();
    let temp_dir = &std::env::temp_dir().display().to_string();
    let output = Command::new("soffice")
        .args([
            "--headless",
            "--convert-to",
            to.to_str(),
            "--outdir",
            temp_dir,
            &input_path_str,
        ])
        .stdout(Stdio::piped())
        .output()
        .await?;
    if output.status.success() {
        let input_path = input_path.with_extension(to.to_str());
        let input_path = input_path
            .file_name()
            .ok_or("no file name")?
            .to_string_lossy();
        let bytes = tokio::fs::read(&format!("{temp_dir}/{input_path}")).await?;
        tokio::fs::remove_file(&format!("{temp_dir}/{input_path}")).await?;
        Ok(bytes)
    } else {
        Err(format!("error {}", String::from_utf8_lossy(&output.stdout)).into())
    }
}

pub enum ConvertType {
    Png,
    // Pdf,
}
impl ConvertType {
    fn to_str(&self) -> &str {
        match self {
            ConvertType::Png => "png",
            //  ConvertType::Pdf => "pdf",
        }
    }
}
