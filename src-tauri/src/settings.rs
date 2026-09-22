use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettings {
    pub privacy: Vec<Setting>, pub appearance: Vec<Setting>, pub tabs: Vec<Setting>,
    pub search: Vec<Setting>, pub browsing: Vec<Setting>, pub downloads: Vec<Setting>,
    pub developer: Vec<Setting>, pub performance: Vec<Setting>, pub accessibility: Vec<Setting>,
    pub advanced: Vec<Setting>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting { pub id:String, pub label:String, pub description:String, pub kind:String, pub default_value:serde_json::Value }
fn group(prefix:&str,names:&[&str])->Vec<Setting>{names.iter().enumerate().map(|(i,n)|Setting{id:format!("{}.{}",prefix,i),label:n.to_string(),description:format!("Voidium preference for {}.",n.to_lowercase()),kind:"toggle".into(),default_value:serde_json::json!(false)}).collect()}
pub fn default_settings()->BrowserSettings{BrowserSettings{
privacy:group("privacy",&["Block third-party cookies","Block trackers","Block fingerprinting scripts","Do Not Track","Clear data on exit","HTTPS-only mode","Block mixed content","Disable prefetch","Disable link prediction","Disable speculative connections","Block popups","Block autoplay","Block WebRTC local addresses","Strip tracking parameters","Block third-party storage","Partition storage"]),
appearance:group("appearance",&["Compact toolbar","Show home button","Show bookmarks bar","Show tab close buttons","Animate tabs","Reduce motion","Use system title bar","Rounded tabs","Dark mode","Follow system theme","Custom accent color","Custom font","Larger text","Page zoom","UI scale","High contrast","Monospace developer font"]),
tabs:group("tabs",&["Restore tabs","Restore pinned tabs","Open new tabs beside current","Reuse closed tab position","Warn before closing window","Mute new tabs","Pin new tab groups","Vertical tabs","Tab search button","Tab previews","Unload inactive tabs","Keep active tab alive","Open external links in new tab","Open downloads in new tab"]),
search:group("search",&["Search suggestions","Local history suggestions","Use HTTPS search","Search on Enter","Highlight search terms","Open results in new tab","Show search engine icon","Keyboard navigation in suggestions"]),
browsing:group("browsing",&["Remember history","Remember form data","Remember passwords","Ask before storing permissions","Show full URL","Always show scheme","Enable reader mode","Save pages for offline use","Restore scroll position","Smooth scrolling","Hardware acceleration","Enable WebGL","Enable WebGPU","Enable service workers","Enable notifications"]),
downloads:group("downloads",&["Ask download location","Open completed downloads","Scan downloaded files","Keep download history","Show download shelf","Limit parallel downloads"]),
developer:group("developer",&["Enable developer tools","Show inspect button","Allow page source editing","Allow live CSS editing","Allow live JavaScript console","Show network panel","Show storage panel","Show accessibility tree","Preserve console log","Disable cache while DevTools is open","Show layout overlays","Show FPS meter"]),
performance:group("performance",&["Lazy load images","Lazy load frames","Reduce background timers","Throttle hidden tabs","Discard inactive pages","Cache static resources","Use GPU rasterization","Limit animation rate","Prefer low memory mode","Reduce pre-rendering","Use parallel style calculation"]),
accessibility:group("accessibility",&["Always show focus ring","Keyboard navigation","Reduce transparency","Increase contrast","Large controls","Readable font mode","Disable animated content","Force minimum font size","Screen reader hints"]),
advanced:group("advanced",&["Enable experimental web APIs","Enable experimental CSS","Enable site isolation","Strict certificate validation","Block invalid certificates","Allow HTTP compatibility mode","Allow local file navigation","Enable custom user scripts","Enable custom user styles","Show internal diagnostics","Write crash reports locally","Verbose engine logging","Developer protocol server","Experimental tab compositor"])
}}
