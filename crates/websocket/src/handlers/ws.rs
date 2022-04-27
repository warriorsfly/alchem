use serde::{Serialize, Deserialize};

// use super::Href;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification{
    /// The title corresponding to the link
    /// **Example value**: "反馈一个问题，需要协助排查"
    /// **Data validation rules**:
    /// - Length range: `0` ～ `512` characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Specific link address
    /// **Example value**: "https://support.feishu.com/internal/foo-bar"
    /// **Data validation rules**:
    /// - Length range: 0 ～ 1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The name of the source of the todo import, which is used to display in the todo center details page. Please provide a dictionary, multi-language name mapping. Supported regional language names: it_it, th_th, ko_kr, es_es, ja_jp, zh_cn, id_id, zh_hk, pt_br, de_de, fr_fr, zh_tw, ru_ru, en_us, hi_in, vi_vn
    /// **Example value**: {"zh_cn": "IT 工作台", "en_us": "IT Workspace"}
    /// **Data validation rules**:Length range: `0` ～ `1024` characters
    #[serde(rename = "platformI18nName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_i18n_name: Option<String>,
    /// Link to the source platform details page of the todo association
    #[serde(rename = "href")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Href>,
}