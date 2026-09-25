use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{env, fs, io::{self, Write}, path::PathBuf};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    theme: String,
    font_family: String,
    font_size: u16,
    private_mode: bool,
    save_history: bool,
    save_cookies: bool,
    javascript: bool,
    images: bool,
    do_not_track: bool,
    developer_tools: bool,
    restore_tabs: bool,
    homepage: String,
}
impl Default for Settings {
    fn default() -> Self { Self { theme:"dark".into(),font_family:"Inter".into(),font_size:16,private_mode:false,save_history:true,save_cookies:true,javascript:true,images:true,do_not_track:true,developer_tools:true,restore_tabs:true,homepage:"https://example.com".into() } }
}
fn config_path()->PathBuf { dirs_next::data_dir().unwrap_or_else(||PathBuf::from(".")).join("Voidium").join("settings.json") }
fn load_settings()->Settings { fs::read_to_string(config_path()).ok().and_then(|s|serde_json::from_str(&s).ok()).unwrap_or_default() }
fn save_settings(s:&Settings)->Result<()> { let p=config_path(); if let Some(parent)=p.parent(){fs::create_dir_all(parent)?;} fs::write(p,serde_json::to_string_pretty(s)?)?; Ok(()) }
fn fetch(url:&str)->Result<String>{ let u=Url::parse(url).context("invalid URL")?; if u.scheme()!="https"{anyhow::bail!("Voidium requires HTTPS.")}; let c=Client::builder().user_agent("Voidium/1.1").build()?; Ok(c.get(u).send()?.error_for_status()?.text()?) }
fn main()->Result<()> {
    if env::args().any(|a|a=="--help"){println!("Voidium 1.1\nLightweight HTTPS browser core.");return Ok(())}
    let mut settings=load_settings();
    println!("VOIDIUM 1.1");
    println!("Native HTTPS core ready.");
    println!("Commands: open <https-url>, search <text>, settings, save, quit");
    let stdin=io::stdin();
    loop {
        print!("\nvoidium> ");io::stdout().flush()?;
        let mut line=String::new();stdin.read_line(&mut line)?;let line=line.trim();
        if line=="quit"||line=="exit"{break}
        if let Some(u)=line.strip_prefix("open "){match fetch(u.trim()){Ok(html)=>println!("Received {} bytes.",html.len()),Err(e)=>eprintln!("Network error: {e:#}")}}
        else if let Some(q)=line.strip_prefix("search "){let u=format!("https://www.google.com/search?q={}",urlencoding::encode(q.trim()));match fetch(&u){Ok(html)=>println!("Received {} bytes.",html.len()),Err(e)=>eprintln!("Search error: {e:#}")}}
        else if line=="settings"{println!("theme={} font={} size={} private={} DNT={} devtools={}",settings.theme,settings.font_family,settings.font_size,settings.private_mode,settings.do_not_track,settings.developer_tools)}
        else if line=="save"{save_settings(&settings)?;println!("Settings saved.")}
        else {println!("Commands: open <https-url>, search <text>, settings, save, quit");}
    }
    settings.private_mode=true;save_settings(&settings)?;Ok(())
}
