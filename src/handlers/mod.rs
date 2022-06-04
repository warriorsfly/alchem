// mod todo;
mod room;
mod user;
// pub mod remind;
// mod ws;
// use serde::{Deserialize, Serialize};
pub(crate) use {room::*, user::*};
// use validator::Validate;

// /// Deadline setting for tasks
// #[derive(Serialize, Deserialize, Default)]
// pub struct Due {
//     /// Timestamp of the deadline (in seconds)
//     /// **Example value**: 1623124318
//     time: i32,
//     /// The time zone corresponding to the deadline, using the IANA Time Zone Database standard, such as Asia/Shanghai
//     /// **Example value**: "Asia/Shanghai"
//     /// **Default value**: `Asia/Shanghai`
//     timezone: String,
//     /// Mark whether the todo is an all-day todo (the deadline for all-day tasks is 0 o'clock of the UTC time of the day)
//     /// **Example value**: false
//     /// **Default value**: `false`
//     is_all_day: bool,
// }

// #[derive(Serialize, Deserialize, Default)]
// pub struct Header {
//     event_id: String,
//     event_type: String,
//     token: String,
//     app_id: String,
//     tenant_key: String,
// }

// #[derive(Debug, Serialize, Deserialize, Default, Validate)]
// pub struct Href {
//     /// The title corresponding to the link
//     /// **Example value**: "反馈一个问题，需要协助排查"
//     /// **Data validation rules**:
//     /// - Length range: `0` ～ `512` characters
//     #[validate(length(min = 0, max = 512))]
//     title: String,
//     /// Specific link address
//     /// **Example value**: "https://support.feishu.com/internal/foo-bar"
//     /// **Data validation rules**:
//     /// - Length range: 0 ～ 1024 characters
//     #[validate(url)]
//     url: String,
// }

// /// Third-party platform source information associated with the todo
// #[derive(Serialize, Deserialize, Validate)]
// pub struct Origin {
//     /// The name of the source of the todo import, which is used to display in the todo center details page. Please provide a dictionary, multi-language name mapping. Supported regional language names: it_it, th_th, ko_kr, es_es, ja_jp, zh_cn, id_id, zh_hk, pt_br, de_de, fr_fr, zh_tw, ru_ru, en_us, hi_in, vi_vn
//     /// **Example value**: {"zh_cn": "IT 工作台", "en_us": "IT Workspace"}
//     /// **Data validation rules**:Length range: `0` ～ `1024` characters
//     #[validate(length(min = 0, max = 1024))]
//     platform_i18n_name: String,

//     /// Link to the source platform details page of the todo association
//     href: Option<Href>,
// }

#[cfg(test)]
mod test {


    // #[tokio::test]
    // async fn test_signup_2000() {
    //     for i in 300000..300000 + 2000_i32 {
    //         let name = i.to_string();
    //         let params = [("name", name), ("password", "12345678".to_string())];

    //         let client = reqwest::Client::new();
    //         let res = client
    //             .post("http://127.0.0.1:3000/api/user/signup")
    //             .form(&params)
    //             .send()
    //             .await
    //             .unwrap();
    //         println!("{:?}", res.text().await.unwrap());
    //     }
    // }

    // #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    // async fn test_join_room_2000() {
    //     use std::collections::HashMap;
    //     for i in 300000..300000 + 2000_i32 {
    //         let mut map = HashMap::new();
    //         map.insert("name", i.to_string());
    //         map.insert("password", "12345678".to_string());
    //         let client = reqwest::Client::new();
    //         let res = client
    //             .post("http://127.0.0.1:3000/api/user/login")
    //             .json(&map)
    //             .send()
    //             .await
    //             .unwrap();
    //         let jwt = serde_json::from_str::<UserToken>(&res.text().await.unwrap()).unwrap();

    //         let client = reqwest::Client::new();
    //         let res = client
    //             .post("http://127.0.0.1:3000/api/room/join/1")
    //             .header("Authorization", format!("Bearer {}", jwt.token))
    //             .send()
    //             .await
    //             .unwrap();
    //         println!("{:?}", res.text().await.unwrap());
    //     }
    // }

    // #[tokio::test]
    // async fn test_ws_2000() {
    //     for i in 300000..300000 + 2000_i32 {
    //         let client = awc::Client::default();
    //         let request = serde_json::json!({
    //             "name":  i.to_string(),
    //             "password": "12345678".to_string()
    //         });

    //         let mut token = client
    //             .post("http://127.0.0.1:3000/api/user/login")
    //             .send_json(&request)
    //             .await
    //             .unwrap();

    //         let toke: UserToken = token.json().await.unwrap();

    //         let (_resp, mut connection) = awc::Client::new()
    //             .ws("http://127.0.0.1:3000/ws")
    //             .set_header("Authorization", format!("Bearer {}", toke.token))
    //             .connect()
    //             .await
    //             .unwrap();
    //         connection
    //             .send(awc::ws::Message::Text("Echo".into()))
    //             .await
    //             .unwrap();
    //     }
    // }
}
