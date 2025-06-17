use crate::{helpers::shorten, models::Browser};
use log::info;

pub fn forward_to_browser(link: &str, browser: Option<Browser>) -> anyhow::Result<()> {
    let _ = match browser {
        Some(browser) => {
            info!("Opening link '{}' in '{}'", shorten(link), browser.name);

            open::with(link, browser.path)
        }
        None => {
            info!("Opening link '{}' in default browser", shorten(link));

            open::that(link)
        }
    };

    Ok(())
}
