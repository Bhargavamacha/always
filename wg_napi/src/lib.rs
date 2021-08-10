use boringtun::crypto::x25519;
use core::str::FromStr;
// use node_bindgen::derive::node_bindgen;
use boringtun::noise::*;
use std::sync::Arc;

// #[node_bindgen]
pub fn wg_start() -> String {
    //   markdown_to_html(md.as_str(), &ComrakOptions::default())
    let private =
        x25519::X25519SecretKey::from_str("AI7WqmJ+7pOuzORYEvwycPZ9dx9aRX+Ce3T1vhzgmnY=").unwrap();
    let public =
        x25519::X25519PublicKey::from_str("pGJECXEPsSHApNkTxEp/F0W24C/zJ84lnvCVrmREnEg=").unwrap();
    let mut peer = Tunn::new(
        Arc::new(private),
        Arc::new(public),
        None,
        None,
        100,
        None,
    )
    .unwrap();
    peer.set_logger(Box::new(|e: &str| eprintln!("client: {}", e)), Verbosity::Trace);
    return "Done".to_string();
}
