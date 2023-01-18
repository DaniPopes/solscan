/// Common derives and other attributes for Solscan API models.
macro_rules! api_models {
    ($($item:item)+) => {$(
        #[derive(Clone, Debug, Default, PartialEq, ::serde::Deserialize, ::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        $item
    )+};
}

/// Easier debugging by retrying deserialization by turning an unknown value into a string.
#[cfg(test)]
macro_rules! test_route {
    ($(#[$attr:meta])* $name:ident : |$client:ident| $route_call:expr => |$x:ident| $block:expr) => {
        test_route!($(#[$attr])* $name : |$client| $route_call => |$x: _| $block);
    };
    ($(#[$attr:meta])* $name:ident : |$client:ident| $route_call:expr => |$x:ident: $ty:ty| $block:expr) => {
        #[::tokio::test]
        $(#[$attr])*
        async fn $name() {
            let $client = crate::Client::new();
            let result: ::core::result::Result<$ty, $crate::ClientError> = $route_call.await;
            match result {
                Ok($x) => $block,
                Err($crate::ClientError::UnknownResponse(value)) => {
                    let s = ::serde_json::to_string(&value).expect("Could not serialize back to a string");
                    eprintln!("--- ERROR ---\n{s}\n\n");
                    ::serde_json::from_str::<'_, $ty>(&s).unwrap()
                }
                e => {
                    e.unwrap();
                }
            }
        }
    };
}
