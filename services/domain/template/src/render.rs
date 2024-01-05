use std::{
    error::Error,
    sync::{Arc, OnceLock},
    time::Duration,
};

use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use minijinja::Environment;
use sequeda_file_upload_client::{DownloadFileRequestUriParams, FileUploadClient};
use sequeda_service_common::IdGenerator;
use sequeda_template_common::{Template, TemplateType};
use serde::Serialize;

static JINJA_ENGINE: OnceLock<Environment<'static>> = OnceLock::new();
static CHROMIUM_TAB: OnceLock<(Browser, Arc<Tab>)> = OnceLock::new();

const CHROMIUM_SANDBOXED: &str = "CHROMIUM_SANDBOXED";

pub fn init() -> Result<(), Box<dyn Error>> {
    tracing::info!("init chromium...");
    get_chromium_tab()?;
    tracing::info!("init jinja...");
    get_jinja_engine()?;
    tracing::info!("init done!");
    Ok(())
}

pub async fn render<T: Serialize>(
    templ: &Template,
    templ_ctx: &T,
    file_client: &FileUploadClient,
    x_user_info: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let templ_file = file_client
        .download(
            x_user_info,
            DownloadFileRequestUriParams {
                id: templ.file_id.clone(),
            },
        )
        .await?;
    let pdf = match templ.template_type {
        TemplateType::Html => html_to_pdf(&templ_file, templ_ctx).await,
    }?;

    Ok(pdf)
}

async fn html_to_pdf<T: Serialize>(templ: &[u8], templ_ctx: &T) -> Result<Vec<u8>, Box<dyn Error>> {
    let engine = get_jinja_engine()?;

    let html = engine.render_str(&String::from_utf8_lossy(templ), templ_ctx)?;
    let tab = get_chromium_tab()?;

    let temp_html_file_path = std::env::temp_dir().join(format!("{}.html", IdGenerator.get()));
    tokio::fs::write(&temp_html_file_path, html).await?;
    let page = format!("file://{}", temp_html_file_path.display());
    tracing::info!("generate pdf from html page {page}");
    let pdf = tab
        .navigate_to(&page)?
        .wait_until_navigated()?
        .print_to_pdf(Default::default())?;
    tokio::fs::remove_file(temp_html_file_path).await?;

    Ok(pdf)
}

fn get_jinja_engine<'a>() -> Result<&'a Environment<'static>, Box<dyn Error>> {
    let engine = {
        if JINJA_ENGINE.get().is_none() {
            JINJA_ENGINE
                .set(Environment::new())
                .map_err(|_env| "could not setup jinja".to_string())?;
        }
        JINJA_ENGINE
            .get()
            .ok_or("could not extract engine from cell")
    }?;
    Ok(engine)
}

fn get_chromium_tab() -> Result<Arc<Tab>, Box<dyn Error>> {
    let (_, tab) = {
        if CHROMIUM_TAB.get().is_none() {
            let options = LaunchOptionsBuilder::default()
                .sandbox(
                    std::env::var(CHROMIUM_SANDBOXED)
                        .map(|v| v.parse::<bool>().unwrap_or(true))
                        .unwrap_or(true),
                )
                .idle_browser_timeout(Duration::MAX)
                .build()
                .map_err(|e| format!("invalid options: {e}"))?;

            let browser = Browser::new(options)?;

            let tab = browser.new_tab()?;

            CHROMIUM_TAB
                .set((browser, tab))
                .map_err(|_tab| "could not setup chromium tab".to_string())?;
        }
        CHROMIUM_TAB.get().ok_or("could not extract tab from cell")
    }?;
    Ok(tab.clone())
}

#[cfg(test)]
mod test {
    use sequeda_service_common::IdGenerator;

    use super::html_to_pdf;

    #[tokio::test]
    async fn test_html_to_pdf() {
        let templ = r#"
        <p>Greeting, {{name}}! You are {{age}} years old!</p>
        <ul>
           {% for stock in stuff.stocks %}
            <li>{{stock}}</li>
           {% endfor %}

        </ul>
        "#;
        let res = html_to_pdf(
            templ.as_bytes(),
            &serde_json::json!({
            "name": "Nordine",
            "age": 35,
            "stuff": {
                "stocks": ["apple", "bananas", "tomatos"]

            }


            }),
        )
        .await
        .unwrap();
        let p = std::env::temp_dir().join(format!("{}.pdf", IdGenerator.get()));
        tokio::fs::write(&p, res).await.unwrap();
        println!("path {p:?}");
    }
}
