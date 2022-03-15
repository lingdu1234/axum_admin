use std::{borrow::Cow, collections::HashMap};

use configs::CFG;
use db::common::client::{ClientInfo, ClientNetInfo, UserAgentInfo};
use headers::HeaderMap;
use poem::web::RemoteAddr;
use user_agent_parser::UserAgentParser;

pub async fn get_client_info(header: HeaderMap, remote_addr: RemoteAddr) -> ClientInfo {
    // 改为 header 中获取

    let user_agent = header.get("user-agent").unwrap().to_str().unwrap();
    let ua = get_user_agent_info(user_agent);
    let ip = get_remote_ip(header, remote_addr);
    let net = get_city_by_ip(&ip).await.unwrap();
    ClientInfo { net, ua }
}

pub fn get_remote_ip(header: HeaderMap, remote_addr: RemoteAddr) -> String {
    let ip = match header.get("X-Forwarded-For") {
        Some(x) => {
            let mut ips = x.to_str().unwrap().split(',');
            ips.next().unwrap().trim().to_string()
        }
        None => match header.get("X-Real-IP") {
            Some(x) => x.to_str().unwrap().to_string(),
            None => remote_addr.to_string(),
        },
    };
    ip
}

pub fn get_user_agent_info(user_agent: &str) -> UserAgentInfo {
    let ua_parser = UserAgentParser::from_path(&CFG.system.user_agent_parser).unwrap();
    let product_v = ua_parser.parse_product(user_agent);
    let os_v = ua_parser.parse_os(user_agent);
    let device_v = ua_parser.parse_device(user_agent);
    let browser = product_v.name.unwrap_or(Cow::Borrowed("")).to_string() + " " + product_v.major.unwrap_or(Cow::Borrowed("")).to_string().as_str();
    let os = os_v.name.unwrap_or(Cow::Borrowed("")).to_string() + " " + os_v.major.unwrap_or(Cow::Borrowed("")).to_string().as_str();
    let device = device_v.name.unwrap_or(Cow::Borrowed("")).to_string();
    UserAgentInfo {
        browser: browser.trim().to_string(),
        os: os.trim().to_string(),
        device,
    }
}

async fn get_city_by_ip(ip: &str) -> Result<ClientNetInfo, Box<dyn std::error::Error>> {
    let url = "http://whois.pconline.com.cn/ipJson.jsp?json=true&ip=".to_string() + ip;
    let resp = reqwest::get(url.as_str()).await?.text_with_charset("utf-8").await?;
    let res = serde_json::from_str::<HashMap<String, String>>(resp.as_str())?;
    let location = format!("{}{}", res["pro"], res["city"]);
    let net_work = res["addr"].split(' ').collect::<Vec<&str>>()[1].to_string();
    Ok(ClientNetInfo {
        ip: res["ip"].to_string(),
        location,
        net_work,
    })
}
