/// Common derives and other attributes for Solscan API models.
macro_rules! api_models {
    ($($item:item)+) => {$(
        #[derive(Clone, Debug, Default, ::serde::Deserialize, ::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        $item
    )+};
}
