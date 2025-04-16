use url::Url;

pub fn sanitize_link(mut link: String) -> String {
    println!("Link: {}", link);
    if !link.starts_with("http") {
        link = "https://".to_owned() + &link
    }

    if let Ok(url) = Url::parse(&link) {
        if let Some(query) = url.query() {
            let splits: Vec<&str> = query.split('&').filter(|d| d.starts_with("url=")).collect();

            if splits.len() == 1 {
                println!("Link: {}", splits[0]);
                if let Ok(url_cow) = urlencoding::decode(&splits[0][4..]) {
                    link = url_cow.into_owned();
                }
            }
        }
    }

    println!("Link: {}", link);

    link.to_owned()
}
