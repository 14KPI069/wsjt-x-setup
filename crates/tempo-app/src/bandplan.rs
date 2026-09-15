/! Tempo's proposed calling-frequency band plan.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandChannel {
    pub band: String,
    pub group: String,
    pub dial_mhz: f64,
    pub mode: String,
    pub label: String,
    pub note: String,
    #[serde(default = "yes")]
    pub tx: bool,
}

fn yes() -> bool {
    true
}

fn ch(band: &str, group: &str, dial_mhz: f64, mode: &str, label: &str, note: &str) -> BandChannel {
    BandChannel {
        band: band.to_string(),
        group: group.to_string(),
        dial_mhz,
        mode: mode.to_string(),
        label: label.to_string(),
        note: note.to_string(),
        tx: true,
    }
}

pub fn canonical_band(token: &str) -> String {
    let t = token.trim();
    t.split('-').next().unwrap_or(t).to_string()
}

pub fn psk_band_plan() -> Vec<BandChannel> {
    all_channels()
}

pub fn rtty_band_plan() -> Vec<BandChannel> {
    all_channels()
}

pub fn ft8_band_plan() -> Vec<BandChannel> {
    all_channels()
}

pub fn ft4_band_plan() -> Vec<BandChannel> {
    all_channels()
}

pub fn band_plan() -> Option<Vec<BandChannel>> {
    Some(all_channels())
}

pub fn band_plan_for<T>(_tier: T) -> Vec<BandChannel> {
    all_channels()
}

pub fn channel_for_dial(dial_mhz: f64) -> Option<BandChannel> {
    all_channels().into_iter().find(|c| {
        (dial_mhz - c.dial_mhz).abs() <= 0.5
    })
}

pub fn band_for_dial(dial_mhz: f64) -> Option<&'static str> {
    match channel_for_dial(dial_mhz)?.band.as_str() {
        "80m" => Some("80m"),
        "40m" => Some("40m"),
        "20m" => Some("20m"),
        "15m" => Some("15m"),
        "10m" => Some("10m"),
        "11m" => Some("11m"),
        "4m" => Some("4m"),
        _ => Some("20m"),
    }
}

pub fn cw_activity_mhz(band: &str) -> Option<f64> {
    match band {
        "11m" => Some(27.550),
        "20m" => Some(14.070),
        "4m" => Some(70.100),
        _ => None,
    }
}

fn all_channels() -> Vec<BandChannel> {
    vec![
        ch(
            "80m",
            "phone",
            3.600,
            "LSB",
            "80m-phone",
            "General / Advanced / Extra phone subband",
        ),
        ch(
            "40m",
            "phone",
            7.125,
            "LSB",
            "40m-phone",
            "General / Advanced / Extra phone subband",
        ),
        ch(
            "20m",
            "phone",
            14.225,
            "USB",
            "20m-phone",
            "General / Advanced / Extra phone subband",
        ),
        ch(
            "15m",
            "phone",
            21.300,
            "USB",
            "15m-phone",
            "General / Advanced / Extra phone subband",
        ),
        ch(
            "10m",
            "phone",
            28.350,
            "USB",
            "10m-phone",
            "General / Advanced / Extra phone subband",
        ),
        ch(
            "11m",
            "data",
            27.555,
            "USB",
            "11m-dx",
            "International 11m call channel",
        ),
        ch(
            "4m",
            "data",
            70.100,
            "USB",
            "4m-dx",
            "International 4m call channel",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_band() {
        assert_eq!(canonical_band("2m-fm"), "2m");
        assert_eq!(canonical_band(" 40m-dx "), "40m");
        assert_eq!(canonical_band("80m"), "80m");
    }
}
