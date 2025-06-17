use log::info;

use crate::models::Browser;

pub fn forward_to_browser(link: &str, browser: Option<Browser>) -> anyhow::Result<()> {
    let _ = match browser {
        Some(browser) => {
            info!("Opening link in '{}'", browser.name);

            open::with(link, browser.path)
        }
        None => open::that(link),
    };

    Ok(())
}
