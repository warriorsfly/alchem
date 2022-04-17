// mod todo;
mod user;
mod room;
// mod ws;
use serde::{Deserialize, Serialize};
pub(crate) use {user::*,room::*};
use validator::Validate;

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
