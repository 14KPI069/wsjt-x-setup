//! Tempo's proposed calling-frequency band plan.

use serde::{Deserialize, Serialize};

/// One Tempo calling channel: a band, a recommended dial frequency, and the mode
/// the radio should be in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandChannel {
    /// Band label, e.g. "20m", "2m".
    pub band: String,
    /// Grouping for the UI: "HF" | "VHF" | "UHF".
    pub group: String,
    /// Recommended Tempo calling dial frequency (MHz, suppressed carrier).
    pub dial_mhz: f64,
    /// Rig mode for this channel: "USB" (weak-signal) or "FM" (simplex data).
    pub mode: String,
    /// Display label for the selector, e.g. "2 m · FM simplex".
    pub label: String,
    /// Short note: what it sits near / clearance / privilege flag.
    pub note: String,
    /// May THIS operator's licence class transmit here?
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

pub fn band_plan() -> Option<Vec<BandChannel>> {
    Some(all_channels())
}

pub fn band_plan_for(_tier: &str) -> Vec<BandChannel> {
    all_channels()
}

pub fn channel_for_dial(dial_mhz: f64) -> Option<BandChannel> {
    let channels = all_channels();
    channels.into_iter().find(|ch| {
        let tolerance = 0.5; // Marge de correspondance autour de la fréquence de dial
        (dial_mhz - ch.dial_mhz).abs() <= tolerance
    })
}

pub fn band_for_dial(dial_mhz: f64) -> Option<String> {
    channel_for_dial(dial_mhz).map(|ch| ch.band)
}

pub fn cw_activity_mhz(band: &str) -> Option<(f64, f64)> {
    match band {
        "11m" => Some((27.550, 27.560)),
        "20m" => Some((14.070, 14.080)),
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
