use crate::appstate::AppState;
use crate::is_key_valid;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{Data, Payload};
use actix_web::{web, HttpRequest};
use futures_util::StreamExt;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
// CREATE TABLE `message` (
// `first_name` VARCHAR(255) NOT NULL,
// `last_name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// `phone` VARCHAR(255) NOT NULL,
// `message` VARCHAR(255) NOT NULL,
// `additional_details` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`email`)
// ) ENGINE=InnoDB;

pub async fn send_discord(msg: AppMessage, settings: HashMap<String, String>) {
    let discord_avatar = match settings.get("discord_avatar") {
        None => "Stark",
        Some(a) => a.as_str(),
    };
    if let Some(webhook_url) = settings.get("discord_webhook") {
        let full_msg = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n",
            msg.email,
            msg.first_name,
            msg.last_name,
            msg.phone,
            msg.message,
            msg.additional_details
        );
        if let Err(e) =
            discord_webhook_lib::send_discord(webhook_url, full_msg.as_str(), discord_avatar).await
        {
            println!("{e}")
        }
    }
}

pub async fn send_email_message(msg: AppMessage, settings: HashMap<String, String>) {
    let full_msg = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n",
        msg.email, msg.first_name, msg.last_name, msg.phone, msg.message, msg.additional_details
    );

    let set = settings.clone();
    let msg_d = msg.additional_details.clone();
    let f_msg = full_msg.clone();
    thread::spawn(|| {
        let tk = tokio::runtime::Runtime::new();
        tk.unwrap().block_on(send_email(set, msg_d, f_msg))
    });
}

pub async fn send_message(msg: AppMessage, settings: HashMap<String, String>) {
    if msg.additional_details.contains("discord") {
        send_discord(msg.clone(), settings.clone()).await;
    }
    if msg.additional_details.contains("email") {
        send_email_message(msg.clone(), settings.clone()).await;
    }
    // todo()!
    // store_message();
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppMessage {
    // date_created: String,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    message: String,
    additional_details: String,
}

// curl -XPOST -H'X-API-KEY: somekey' localhost:8202/message/ -d '{"first_name":"James","last_name":"Koonts","email":"james@koonts.net","phone":"7","message":"","additional_details":""}'
pub async fn message_route(
    // name: web::Path<String>,
    mut payload: Payload,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    const MAX_SIZE: usize = 262_144; // max payload size is 256k
    let mut auth = false;
    // verify api_key
    if req.headers().get("X-API-KEY").is_some() {
        if is_key_valid(
            req.headers()
                .get("X-API-KEY")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            data.lock().unwrap().api_keys.lock().unwrap().to_vec(),
        ) {
            auth = true;
        }
    }

    if auth {
        let mut body = web::BytesMut::new();

        while let Some(chunk) = payload.next().await {
            let chunk = chunk.unwrap();
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                return ErrorBadRequest("overflow").to_string();
            }
            body.extend_from_slice(&chunk);
        }

        // println!("{:?}", body);
        if let Ok(message) = serde_json::from_slice::<AppMessage>(&body) {
            let settings = data.lock().unwrap().settings.lock().unwrap().clone();
            send_message(message, settings).await;
            // println!("{:?}", message);
            "ok\n".to_string()
        } else {
            "failure to deserialize\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub async fn send_email(settings_map: HashMap<String, String>, subject: String, message: String) {
    if settings_map
        .get("smtp_enabled")
        .expect("could not get smtp_enabled")
        .parse::<bool>()
        .unwrap()
    {
        let host: String = settings_map
            .get("smtp_host")
            .expect("could not get smtp_host")
            .to_string();
        // let port: u32 = settings_map.get("smtp_port").expect("could not get smtp_port").parse::<u32>().unwrap();
        // let require_auth: bool = settings_map.get("smtp_require_auth").expect("could not get smtp_require_auth").parse::<bool>().unwrap();
        let username: String = settings_map
            .get("smtp_username")
            .expect("could not get smtp_username")
            .to_string();
        let password: String = settings_map
            .get("smtp_password")
            .expect("could not get smtp_password")
            .to_string();
        let smtp_from: String = settings_map
            .get("smtp_from")
            .expect("could not get smtp_from")
            .to_string();

        // println!("{:#?}", settings_map);
        let mut all_recipients: Vec<String> = vec![];

        settings_map
            .iter()
            .filter_map(|e| match e.0.contains("smtp_recipient") {
                true => Some(e.1),
                false => None,
            })
            .collect::<Vec<_>>()
            .iter()
            .for_each(|e| all_recipients.push(e.to_string()));

        // if message contains ::PAGERDUTY:: add pgduty email to recipients list

        println!("Emailing:: {:#?}", all_recipients);

        all_recipients.iter().for_each(|recipient| {
            let email = Message::builder()
                .from(smtp_from.parse().unwrap())
                .reply_to(smtp_from.parse().unwrap())
                .to(recipient.clone().parse().unwrap())
                .subject(subject.clone())
                .header(ContentType::TEXT_PLAIN)
                .body(String::from(message.clone()))
                .unwrap();

            let creds = Credentials::new(username.to_owned(), password.to_owned());

            // Open a remote connection to gmail
            let mailer = SmtpTransport::relay(host.as_str())
                .unwrap()
                .credentials(creds)
                .build();

            // // Send the email
            match mailer.send(&email) {
                Ok(_) => println!("Email sent successfully!"),
                Err(e) => println!("Could not send email: {e:?}"),
            }
            thread::sleep(Duration::new(1, 500000000)); // sleep for 1.5 seconds
        });
    }
}
