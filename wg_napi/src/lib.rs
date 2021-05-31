use boringtun::crypto::x25519;
use boringtun::noise::Tunn;
use core::str::FromStr;
// use node_bindgen::derive::node_bindgen;
use std::option::Option;
use std::sync::Arc;

pub fn logger(log: String){
    println!("{}", log);
}
// #[node_bindgen]
pub fn wg_start() -> String {
    //   markdown_to_html(md.as_str(), &ComrakOptions::default())
    let private =
        x25519::X25519SecretKey::from_str("AI7WqmJ+7pOuzORYEvwycPZ9dx9aRX+Ce3T1vhzgmnY=").unwrap();
    let public =
        x25519::X25519PublicKey::from_str("pGJECXEPsSHApNkTxEp/F0W24C/zJ84lnvCVrmREnEg=").unwrap();
    let wg = Tunn::new(
        Arc::new(private),
        Arc::new(public),
        Option::default(),
        Option::default(),
        0,
        Option::default()
        // peer_static_public: Arc<X25519PublicKey>,
        // preshared_key: Option<[u8; 32]>,
        // persistent_keepalive: Option<u16>,
        // index: u32,
        // rate_limiter: Option<Arc<RateLimiter>>,
    );
    wg.unwrap().set_logger(Box::new(), boringtun::noise::Verbosity::Trace);
    return "Done".to_string();
}
