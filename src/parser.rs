use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

const fn _default_true() -> bool {
    true
}

#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::struct_field_names)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "_default_true")]
    sign_uki: bool,

    #[serde(default = "_default_true")]
    remove_leftovers: bool,

    #[serde(default)]
    cmdline_per_kernel: bool,

    // TODO: upstream defaults to the arch splash image, but I should just make
    // this Option and have it not set an image if possible
    splash: String,

    #[serde(default)]
    generate_fallback: bool,

    // TODO: Throw a warning if you disable fallback generation but specify it
    // in the initramfs_config
    // FIXME: This should specifically check for fallback and default only in
    // the values
    initramfs_config: Dictionary,

    #[serde(default)]
    efistub: bool,

    efistub_config: Option<Dictionary>,

    #[serde(default)]
    sbsign: bool,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_csv")]
    sbsign_config: Option<Dictionary>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dictionary {
    #[serde(flatten)]
    inner: HashMap<String, DictValues>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum DictValues {
    VecStr(Vec<String>),
    String(String),
    Bool(bool),
}

fn deserialize_csv<'de, D>(deserializer: D) -> Result<Option<Dictionary>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut buf: HashMap<String, DictValues> = HashMap::deserialize(deserializer)?;
    buf.entry(String::from("pcr_banks")).and_modify(|val| {
        if let DictValues::String(value) = val {
            *val = DictValues::VecStr(value.split(',').map(String::from).collect());
        }
    });

    Ok(Some(Dictionary { inner: buf }))
}
