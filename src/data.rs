const API_URL: &str = "https://sg-hkrpg-api.hoyoverse.com/common/apicdkey/api/webExchangeCdkey";



// pub enum Region{
//     Asia,
//     NorthAmerica,
//     Europe,
//     China,
// }

// todo: region-specific API URLs
// Only works for Asia/Singapore region at the moment, aka the "Asia" server
// 
// Need to implement switching between NA, EU, CN and Asia servers

use std::time::SystemTime;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct GiftClient {
    #[serde(rename = "_MHYUUID")]
    pub mhyuuid: Uuid,
    #[serde(rename = "mi18nLang")]
    pub mi18n_lang: String,
    #[serde(rename = "DEVICEFP")]
    pub devicefp: String,
    #[serde(rename = "DEVICEFP_SEED_ID")]
    pub devicefp_seed_id: String,
    #[serde(rename = "DEVICEFP_SEED_TIME")]
    pub devicefp_seed_time: String,
    #[serde(rename = "cookie_token_v2")]
    pub cookie_token_v2: String,
    #[serde(rename = "account_mid_v2")]
    pub account_mid_v2: String,
    #[serde(rename = "account_id_v2")]
    pub account_id_v2: String,
    #[serde(rename = "HYV_LOGIN_PLATFORM_OPTIONAL_AGREEMENT")]
    pub hyv_login_platform_optional_agreement: String,
    #[serde(rename = "HYV_LOGIN_PLATFORM_LOAD_TIMEOUT")]
    pub hyv_login_platform_load_timeout: String,
    #[serde(rename = "HYV_LOGIN_PLATFORM_TRACKING_MAP")]
    pub hyv_login_platform_tracking_map: String,
    #[serde(rename = "HYV_LOGIN_PLATFORM_LIFECYCLE_ID")]
    pub hyv_login_platform_lifecycle_id: String,
}

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

impl GiftClient {
    pub fn cookie_string(&self) -> String {
        format!(
            "_MHYUUID={}; mi18nLang={}; DEVICEFP={}; DEVICEFP_SEED_ID={}; DEVICEFP_SEED_TIME={}; cookie_token_v2={}; account_mid_v2={}; account_id_v2={}; HYV_LOGIN_PLATFORM_OPTIONAL_AGREEMENT={}; HYV_LOGIN_PLATFORM_LOAD_TIMEOUT={}; HYV_LOGIN_PLATFORM_TRACKING_MAP={}; HYV_LOGIN_PLATFORM_LIFECYCLE_ID={}",
            self.mhyuuid,
            self.mi18n_lang,
            self.devicefp,
            self.devicefp_seed_id,
            self.devicefp_seed_time,
            self.cookie_token_v2,
            self.account_mid_v2,
            self.account_id_v2,
            self.hyv_login_platform_optional_agreement,
            self.hyv_login_platform_load_timeout,
            self.hyv_login_platform_tracking_map,
            self.hyv_login_platform_lifecycle_id
        )
    }

    pub fn from_cookie_string(cookie: &str) -> Self {
        let mut mhyuuid = Uuid::nil();
        let mut mi18n_lang = String::new();
        let mut devicefp = String::new();
        let mut devicefp_seed_id = String::new();
        let mut devicefp_seed_time = String::new();
        let mut cookie_token_v2 = String::new();
        let mut account_mid_v2 = String::new();
        let mut account_id_v2 = String::new();
        let mut hyv_login_platform_optional_agreement = String::new();
        let mut hyv_login_platform_load_timeout = String::new();
        let mut hyv_login_platform_tracking_map = String::new();
        let mut hyv_login_platform_lifecycle_id = String::new();

        for cookie in cookie
            .trim_start_matches("Cookie: ")
            .trim_end_matches("\r\n")
            .split("; ")
        {
            let mut parts = cookie.split("=");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            match key {
                "_MHYUUID" => mhyuuid = Uuid::parse_str(value).unwrap(),
                "mi18nLang" => mi18n_lang = value.to_string(),
                "DEVICEFP" => devicefp = value.to_string(),
                "DEVICEFP_SEED_ID" => devicefp_seed_id = value.to_string(),
                "DEVICEFP_SEED_TIME" => devicefp_seed_time = value.to_string(),
                "cookie_token_v2" => cookie_token_v2 = value.to_string(),
                "account_mid_v2" => account_mid_v2 = value.to_string(),
                "account_id_v2" => account_id_v2 = value.to_string(),
                "HYV_LOGIN_PLATFORM_OPTIONAL_AGREEMENT" => {
                    hyv_login_platform_optional_agreement = value.to_string()
                }
                "HYV_LOGIN_PLATFORM_LOAD_TIMEOUT" => {
                    hyv_login_platform_load_timeout = value.to_string()
                }
                "HYV_LOGIN_PLATFORM_TRACKING_MAP" => {
                    hyv_login_platform_tracking_map = value.to_string()
                }
                "HYV_LOGIN_PLATFORM_LIFECYCLE_ID" => {
                    hyv_login_platform_lifecycle_id = value.to_string()
                }
                _ => {}
            }
        }

        Self {
            mhyuuid,
            mi18n_lang,
            devicefp,
            devicefp_seed_id,
            devicefp_seed_time,
            cookie_token_v2,
            account_mid_v2,
            account_id_v2,
            hyv_login_platform_optional_agreement,
            hyv_login_platform_load_timeout,
            hyv_login_platform_tracking_map,
            hyv_login_platform_lifecycle_id,
        }
    }

    pub fn client(&self) -> Client {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Cookie",
            reqwest::header::HeaderValue::from_str(&self.cookie_string()).unwrap(),
        );
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0",
            ),
        );
        headers.insert(
            "content-type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap()
    }

    pub async fn redeem(&self, cdkey: &str, uid: &str) -> error::Result<error::Return> {
        let response = self
            .client()
            .get(API_URL)
            .query(&[
                // UNIX timestamp
                ("t", get_sys_time_in_secs().to_string().as_str()),
                ("lang", "en"),
                ("game_biz", "hkrpg_global"),
                ("uid", uid),
                ("region", "prod_official_asia"),
                ("cdkey", cdkey),
            ])
            .send()
            .await?;
        let text = response.text().await?;
        error::Return::import(&text)
    }
}
