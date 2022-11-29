use alw_utils::validate::ValidatedForm;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{Due, Origin};

#[derive(Deserialize, Validate)]
pub struct Todo {
    /// Todo Title. When creating a todo, the Feishu server will treat it as a topic-free todo if no title is filled
    /// **Example value**: "每天喝八杯水，保持身心愉悦"
    /// **Data validation rules**:
    /// - Length range: `1` ～ `256` characters
    #[validate(length(min = 1, max = 256))]
    summary: String,
    /// Todo remarks
    /// **Example value**: "多吃水果，多运动，健康生活，快乐工作。"
    /// **Data validation rules**:
    /// - Length range: `0` ～ `65536` characters
    #[validate(length(max = 65536))]
    discription: String,
    /// The access party can customize the subsidiary information binary format, using base64 encoding, and the resolution method is determined by the access party itself
    /// **Example value**: "dGVzdA=="
    /// **Data validation rules**:
    /// - Length range: `0` ～ `65536` characters
    #[validate(length(max = 65536))]
    extra: String,
    /// Deadline setting for todos
    due: Option<Due>,
    /// Third-party platform source information associated with the todo
    origin: Option<Origin>,
    /// This field is used to control whether the todo is editable in the Feishu todo center. The default is false. If it is true, the third party needs to consider whether it needs to access events to receive the change information of the todo in the todo center
    /// **Example value**: true
    /// **Default value**: `false`
    editable: bool,
    /// This field is used to store custom data that third parties need to pass through to the end, in Json format. In the value example, custom_complete field stores the jump link (href) or prompt message (tip) of the "Complete" button. PC, ios, android can be customized. The key of the tip field is the language type, and the value is the prompt message. The language type can be increased or decreased by itself. The language name of each region supported is it_it, th_th, ko_kr, es_es, ja_jp, zh_cn, id_id, zh_hk, pt_br, de_de, fr_fr, zh_tw, ru_ru, en_us, hi_in, vi_vn. The priority of href is higher than tip, and only jump without prompt when href and tip are not empty at the same time. Links and prompt messages can be customized, and the rest of the keys need to be passed according to the structure in the example
    /// **Example value**: "{"custom_complete":{"android":{"href":"https://www.google.com.hk/","tip":{"zh_cn":"你好","en_us":"hello"}},"ios":{"href":"https://www.google.com.hk/","tip":{"zh_cn":"你好","en_us":"hello"}},"pc":{"href":"https://www.google.com.hk/","tip":{"zh_cn":"你好","en_us":"hello"}}}}"
    /// **Data validation rules**:
    /// - Length range:`0` ～ `65536` characters
    #[validate(length(max = 65536))]
    custom: String,
}

#[derive(Serialize)]
pub struct TodoJson {
    /// Todo ID, issued by the Feishu todo server
    id: String,
    /// Todo Title. When creating a todo, the Feishu server will treat it as a topic-free todo if no title is filled
    summary: String,
    /// Todo remarks
    discription: String,
    /// The access party can customize the subsidiary information binary format, using base64 encoding, and the resolution method is determined by the access party itself
    extra: String,
    /// Deadline setting for todos
    due: Option<Due>,
    /// Third-party platform source information associated with the todo
    origin: Option<Origin>,
    /// This field is used to control whether the todo is editable in the Feishu todo center. The default is false. If it is true, the third party needs to consider whether it needs to access events to receive the change information of the todo in the todo center
    editable: bool,
    /// This field is used to store custom data that third parties need to pass through to the end, in Json format. In the value example, custom_complete field stores the jump link (href) or prompt message (tip) of the "Complete" button. PC, ios, android can be customized. The key of the tip field is the language type, and the value is the prompt message. The language type can be increased or decreased by itself. The language name of each region supported is it_it, th_th, ko_kr, es_es, ja_jp, zh_cn, id_id, zh_hk, pt_br, de_de, fr_fr, zh_tw, ru_ru, en_us, hi_in, vi_vn. The priority of href is higher than tip, and only jump without prompt when href and tip are not empty at the same time. Links and prompt messages can be customized, and the rest of the keys need to be passed according to the structure in the example
    custom: String,
    /// The source of the todo created
    /// Optional values are:
    /// - 0: Unknown type
    /// - 1: Source todo center
    /// - 2: Source message to todo
    /// - 3: Source DOC
    /// - 4: Source DOC product
    /// - 5: Source PANO
    /// - 6: Source
    /// - 7: Source tenant_access_token created
    /// - 8: Source user_access_token created
    source: i32,
}

async fn create_todo(todo: ValidatedForm<Todo>) {}
#[derive(Deserialize)]
pub struct PatchTodoInput {
    todo: Todo,
    update_fields: Vec<String>,
}

// async fn patch_todo(todo: ValidatedJson<PatchTodoInput>, todo_id: Query<String>) {}

// async fn complete_todo(todo_id: Query<String>) {}

// async fn uncomplate_todo(todo_id: Query<String>) {}

// async fn delate_todo(todo_id: Query<String>) {}

// async fn get_todos(
//     page_size: Query<Option<i32>>,
//     page_index: Query<Option<i32>>,
//     user_id_type: Query<Option<i32>>,
// ) {
// }

// async fn get_todo_detail(todo_id: Query<String>) {}
