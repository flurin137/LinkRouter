use log::info;

use crate::models::Browser;

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

fn shorten(string: &str) -> String {
    if string.len() < 50 {
        return string.into();
    }

    let mut shortened = String::from(&string[0..47]);
    shortened.push_str("...");
    shortened
}
