#[macro_export]
macro_rules! spawn {
    ($page: expr, $tx: expr, |$page_ident: ident, $tx_ident: ident| $block: block) => {{
        let $page_ident = std::sync::Arc::clone(&$page);
        let $tx_ident = $tx.clone();
        tokio::spawn(async move $block)
    }};
}
