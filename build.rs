use anyhow::Result;
use convert_case::{Case, Casing};
use serde::Serialize;
use std::{env, fs::File, path::PathBuf};
use tera::Tera;

const FA_VERSION: &str = "6.5.2";

mod api {
    use anyhow::Result;
    use serde::Deserialize;
    use std::collections::HashMap;

    use crate::FA_VERSION;

    static URL: &str = "https://api.fontawesome.com";

    #[derive(Deserialize)]
    struct Response {
        data: Data,
    }

    #[derive(Deserialize)]
    struct Data {
        release: Release,
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct Release {
        pub(crate) icons: Vec<Icon>,
        pub(crate) version: String,
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct Icon {
        pub(crate) id: String,
        pub(crate) membership: Membership,
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct Membership {
        pub(crate) free: Vec<String>,
    }

    pub(crate) async fn get_release() -> Result<Release> {
        let client = reqwest::Client::new();
        let param = format!(
            "\
            query {{
                release (version: \"{}\") {{
                    version,
                    icons {{
                        id,
                        label,
                        unicode,
                        membership {{
                            free,
                        }},
                    }},
                }},
            }},
        ",
            FA_VERSION
        );
        let resp: Response = client
            .post(URL)
            .json(&HashMap::from([("query", param)]))
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.data.release)
    }
}

#[derive(Serialize, Clone)]
struct Icon {
    ident: String,
    id: String,
    is_free: bool,
    membership: String,
}

impl Icon {
    fn format_name_to_identifier(name: &str) -> String {
        let mut name = name.to_case(Case::Pascal);
        if name
            .chars()
            .next()
            .expect("name should not be empty")
            .is_ascii_digit()
        {
            name.insert(0, '_');
        }
        name.to_uppercase()
    }
}

impl From<api::Icon> for Icon {
    fn from(icon: api::Icon) -> Self {
        Self {
            ident: Self::format_name_to_identifier(&icon.id),
            id: icon.id,
            is_free: !icon.membership.free.is_empty(),
            membership: icon
                .membership
                .free
                .first()
                .unwrap_or(&"Error".to_string())
                .clone(),
        }
    }
}

struct Release {
    version_str: String,
    icons: Vec<Icon>,
}

async fn get_release() -> Result<Release> {
    let api_release = api::get_release().await?;
    Ok(Release {
        version_str: api_release.version,
        icons: api_release.icons.into_iter().map(Into::into).collect(),
    })
}

fn render_template(
    templates: &Tera,
    context: &tera::Context,
    template_name: &str,
    target_name: &str,
) -> Result<()> {
    let target_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let target_path = target_dir.join(target_name);

    let target_file = File::create(&target_path)?;
    templates.render_to(template_name, context, target_file)?;
    println!("Output file written to {:?}", &target_path);
    println!("cargo:rerun-if-changed=src/{template_name}");
    Ok(())
}

fn render_templates(release: &Release) -> Result<()> {
    let mut source_dir: PathBuf = env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    source_dir.push("src");
    let templates = Tera::new(source_dir.join("*.tera").to_str().unwrap())?;

    let mut context = tera::Context::new();
    let mut icons: Vec<CustomIcon> = vec![];
    let res = release.icons.clone();
    for ele in res.into_iter().filter(|ele| ele.is_free) {
        let url = build_url(&release.version_str.to_string(), &ele.membership, &ele.id);
        let icon = CustomIcon {
            ident: ele.ident.clone(),
            url,
        };
        icons.push(icon);
    }
    context.insert("icons", &icons);

    render_template(&templates, &context, "icon.rs.tera", "icon.rs")?;
    Ok(())
}
#[derive(Serialize)]
struct CustomIcon {
    ident: String,
    url: String,
}

fn build_url(version: &str, membership: &str, id: &str) -> String {
    format!(
        "https://site-assets.fontawesome.com/releases/v{}/svgs/{}/{}.svg",
        version, membership, id
    )
    .to_string()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let release = get_release().await?;
    println!("cargo:rerun-if-changed=build.rs");
    render_templates(&release)?;
    Ok(())
}
