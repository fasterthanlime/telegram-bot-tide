use std::{env, error::Error};
use telegram_bot::{requests::send_message::CanSendMessage, Api, ChatId};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Civ6Notification {
    #[serde(rename = "value1")]
    game_name: String,
    #[serde(rename = "value2")]
    current_player: String,
    #[serde(rename = "value3")]
    turn_number: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rt = tokio::runtime::Handle::current();

    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let tg_chat = || -> Result<ChatId, Box<dyn Error>> {
        Ok(env::var("TELEGRAM_CHAT_ID")?.parse::<i64>()?.into())
    }()
    .expect("TELEGRAM_CHAT_ID should be a valid ChatId");
    let listen_addr = env::var("LISTEN_ADDR").expect("specify LISTEN_ADDR in env");
    let tg_token = env::var("TELEGRAM_BOT_TOKEN").expect("specify TELEGRAM_BOT_TOKEN in env");

    let mut app = tide::with_state((Api::new(&tg_token), tg_chat, rt));
    app.at("/").post(
        |mut req: tide::Request<(Api, ChatId, tokio::runtime::Handle)>| async move {
            let civ: Option<Civ6Notification> = req.body_json().await.ok();
            let (tg, chat, rt) = req.state();
            let tg = tg.clone();
            match civ {
                None => tide::Response::new(400),
                Some(civ) => {
                    let res = rt
                        .spawn(tg.send(chat.text(format!(
                            "Player {} please report to game {} turn {}",
                            civ.current_player, civ.game_name, civ.turn_number
                        ))))
                        .await
                        .unwrap();
                    match res {
                        Ok(_) => tide::Response::new(200),
                        Err(e) => {
                            log::error!(
                                "While reporting on {:?} to chat {:?}, an error occured: {:?}",
                                civ,
                                chat,
                                e
                            );
                            tide::Response::new(500)
                        }
                    }
                }
            }
        },
    );
    app.listen(listen_addr).await?;

    Ok(())
}
