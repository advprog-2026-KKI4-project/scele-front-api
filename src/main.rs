use std::{sync::Mutex, thread, time::Duration};

use actix_web::{Responder, Result, get, web};
use chrono::{DateTime, Utc};
use rand::Rng;
use scele_frontapi::get_frontpage;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct AnnouncementResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub date_time: DateTime<Utc>,
}

struct ServerState {
    pub request_count: Mutex<u8>,
}

fn parse_frontpage(page: Html) -> Vec<AnnouncementResponse> {
    let selector = Selector::parse("article").unwrap();
    let elements_iterator = page.select(&selector);
    let mut announcements = Vec::<AnnouncementResponse>::new();

    for element in elements_iterator {
        let id = String::from(element.attr("id").unwrap());
        let title = element
            .select(&Selector::parse("h3").unwrap())
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap();
        let author = element
            .select(&Selector::parse("a").unwrap())
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap();
        
        let announcement = AnnouncementResponse {
            id,
            title,
            author,
            date_time: Utc::now(), // TODO: Parse the time value from the HTML
        };

        announcements.push(announcement);
    }

    return announcements;
}

#[get("/announcements")]
async fn get_all_announcements(data: web::Data<ServerState>) -> Result<impl Responder> {
    let page = get_frontpage("https://scele.cs.ui.ac.id").unwrap();
    let announcements = parse_frontpage(page);

    {
        let mut request_count = data.request_count.lock().unwrap();
        let delay_ms = rand::thread_rng().gen_range(0..1000_u64);
        thread::sleep(Duration::from_millis(delay_ms));
        *request_count += 1;
        println!("Request count: {}", *request_count);
        }

    Ok(web::Json(announcements))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
let state = web::Data::new(ServerState {
        request_count: Mutex::new(0),
    });

    use actix_web::{App, HttpServer};

    HttpServer::new(move || App::new()
        .app_data(state.clone())
        .service(get_all_announcements))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
