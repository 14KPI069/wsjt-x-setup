//! Tempo's proposed calling-frequency band plan.
//!
//! Tempo is a NEW narrow weak-signal text mode (FT1 ~150 Hz, DX1 ~50 Hz), so it
//! must **not** sit on the established FT8 / FT4 / JS8 / WSPR / PSK watering holes
//! (mutual QRM), and it must stay clear of CW activity and the VHF/UHF FM calling
//! / satellite / repeater segments.
//!
//! Every entry here was chosen so that — for a USB signal with the usual ~1500 Hz
//! audio offset, i.e. an emission ~1.5 kHz above the dial — the **emission falls
//! inside the US General-class data privileges** (General has the HF data
//! sub-bands and full privileges on 160 m / 6 m and band-wide data above 50 MHz),
//! and sits clear of the CW calling frequencies. These are **proposed, editable
//! defaults** to coordinate with the community — the operator can override any
//! frequency manually.
//!
//! HF placement = "upper shoulder of the digital cluster" (a few kHz above
//! FT8/JS8/FT4, below WSPR). VHF/UHF = a USB weak-signal calling freq and, where
//! it fits a band-plan digital/experimental segment, an FM-simplex DATA channel
//! for FM-HT users — always offset clear of the FM national calling freqs
//! (146.520 / 446.000 / 223.500), APRS, satellite, and repeater sub-bands.

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
    /// May THIS operator's licence class transmit here? (#184, akhepcat)
    ///
    /// ⚠️ FALSE MEANS RECEIVE-ONLY, NOT HIDDEN. The band dropdowns used to drop a band the
    /// class held no transmit segment for, which applied a TRANSMIT rule to a TUNING list:
    /// no licence restricts LISTENING, and the radio itself will happily tune there. A US
    /// General was therefore unable to select 4 m at all, rather than being able to listen
    /// and being refused the over.
    ///
    /// This field is DISPLAY ONLY and the transmit gate does not read it —
    /// [`crate::privileges::tx_allowed`] is still the only thing that decides whether an
    /// over may be keyed, and it is unchanged. Defaults true so every existing plan entry
    /// and any stored JSON keeps its current meaning.
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

/// The AWARD/ADIF band identity for a band-plan channel token. Channel ids may
/// carry a suffix that distinguishes CHANNELS on one band ("2m-fm", "6m-2",
/// "2m-call", "40m-dx", "80m-eu") — presentation ids, never band identities.
/// The suffix must not reach stored state: `settings.band` feeds
/// `QsoRecord.band`, the ADIF file and every upload verbatim, and the award/
/// interop readers accept only the base label. THE one place the suffix is
/// stripped — call this at the state boundary rather than hand-splitting.
pub fn canonical_band(token: &str) -> String {
    let t = token.trim();
    t.split('-').next().unwrap_or(t).to_string()
}

/// The proposed Tempo band plan — verified US General-legal + CW-clear (judged on
/// US 60 m regional channels where applicable, and General CW/phone subbands).
pub fn psk_band_plan() -> Vec<BandChannel> {
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

    #[test]
    fn cw_activity_is_inside_band_and_off_the_edge() {
        let psk = psk_band_plan();
        let dial = |band: &str| -> f64 {
            psk.iter()
                .find(|c| c.band == band)
                .map(|c| c.dial_mhz)
                .unwrap_or(0.0)
        };
        let off = 0.001;

        assert!(
            !tx_allowed(
                LicenseClass::Technician,
                dial("40m") + off,
                OperatingMode::Cw
            ),
            "a Technician must not key 40 m CW"
        );
        assert!(
            tx_allowed(
                LicenseClass::Technician,
                dial("10m") + off,
                OperatingMode::Cw
            ),
            "10 m CW is inside Technician privileges"
        );
        assert!(
            tx_allowed(
                LicenseClass::Technician,
                dial("6m") + off,
                OperatingMode::Cw
            ),
            "6 m CW is inside Technician privileges"
        );
        assert!(
            !tx_allowed(LicenseClass::Extra, 14.300, OperatingMode::Cw),
            "a Extra must not key 14.300 CW"
        );
    }
}
