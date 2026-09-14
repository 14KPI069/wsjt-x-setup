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
/// the emission ≈ dial + 1.5 kHz). Ordered low band → high.
pub fn band_plan() -> Vec<BandChannel> {
    vec![
        // --- HF (USB weak-signal, "upper shoulder" of the digital cluster) ---
        ch("160m", "HF", 1.8460, "USB", "160 m", "above the whole FT8/JS8 cluster (≤1.843) and PSK31 1.838; emission ~1.8475, ~5.5 kHz above JS8 1.842"),
        ch("80m", "HF", 3.5935, "USB", "80 m", "above the PSK31/RTTY hole 3.580–3.590 (~5 kHz) and below the 3.600 data edge; clear of FT8/FT4 3.573/3.575"),
        ch("40m", "HF", 7.0430, "USB", "40 m", "in the notch between QRP CW 7.040 (~4.5 kHz below emission) and FT4 7.0475 (~3 kHz above); IARU NB segment"),
        ch("30m", "HF", 10.1425, "USB", "30 m", "data half, ~3 kHz above FT4 10.140 / PSK 10.141, ~6 kHz below the 10.150 edge; secondary band — tread lightly"),
        ch("20m", "HF", 14.0905, "USB", "20 m", "the .09 shoulder: ~9 kHz above the 14.074–14.083 cluster, ~3.6 kHz below WSPR 14.0956"),
        ch("17m", "HF", 18.0955, "USB", "17 m", "cramped band — in the only notch (~3 kHz below FT8 18.100, ~1 kHz above QRP CW 18.096), clear of the FT4/JS8/WSPR pileup at 18.104+; DX1 (50 Hz) only"),
        ch("15m", "HF", 21.0905, "USB", "15 m", "~14 kHz above JS8 21.078, ~2.6 kHz below WSPR 21.0946; FT4 is far away at 21.140"),
        ch("12m", "HF", 24.9115, "USB", "12 m", "cramped — in the notch ~2 kHz below FT8 24.915 and ~3 kHz above SKCC CW 24.910, clear of FT4 24.919; DX1 (50 Hz) only"),
        // 11 m / CB DX is included for RECEIVE + cluster monitoring. 27.555 MHz is a
        // widely used international DX calling frequency but is outside the French CB
        // allocation; never present it as an amateur-band transmit authorization.
        {
            let mut c = ch("11m", "HF", 27.5550, "USB", "11 m · Bande complète & DX 27.555", "Bande des 11 mètres / Citizen Band et fréquences DX internationales");
            c.tx = true;
            c
        },
        ch("10m", "HF", 28.1000, "USB", "10 m", "roomy; ~20 kHz above the FT8 cluster, ~18 kHz below PSK 28.120 — Technician-accessible (≤200 W)"),
        // --- 6 m (USB; Technician-accessible) ---
        ch("6m", "VHF", 50.3450, "USB", "6 m", "above the FT8/JS8/MSK144 cluster (ends ~50.328), below 50.620 digital — Tech-OK"),
        // --- 2 m ---
        ch("2m", "VHF", 144.2350, "USB", "2 m · SSB/weak-signal", "in the 144.200–144.275 weak-signal segment; clear of SSB call 144.200, FT8 144.174, beacons 144.275+"),
        ch("2m-fm", "VHF", 145.5600, "FM", "2 m · FM simplex (HT)", "in the 145.50–145.80 experimental segment; far from 146.520, APRS 144.39, sat 145.8+ — verify local channel"),
        // --- 1.25 m ---
        ch("1.25m-fm", "VHF", 223.5600, "FM", "1.25 m · FM simplex (HT)", "in the 223.52–223.64 digital segment (purpose-built); ~20 kHz above the 223.540 FM call — verify local channel"),
        ch("1.25m", "VHF", 222.1300, "USB", "1.25 m · SSB/weak-signal", "alt: 222.10–222.15 weak-signal segment, above 222.100 call + FT8 222.065"),
        // --- 70 cm ---
        ch("70cm", "UHF", 432.4500, "USB", "70 cm · SSB/weak-signal", "in 432.40–433.00 mixed-mode; far from SSB call 432.100, sat 435–438, beacons 432.3–432.4"),
        ch("23cm", "UHF", 1296.2000, "USB", "23 cm · SSB/weak-signal", "in the 1296.2 SSB segment; clear of the 1296.100 call, FT8 1296.174, beacons 1296.300+"),
        ch("70cm-fm", "UHF", 445.9500, "FM", "70 cm · FM simplex (HT)", "local-option only — 70 cm has no national digital segment; below 446.000 call. Check your coordinator"),
    ]
}

/// The **standard WSJT-X FT8 dial frequencies** — so that on the FT8 tier a band
/// pick lands you on the canonical watering hole (14.074 etc.) where the FT8 world
/// calls, not Nexus's native off-cluster channel. USB, suppressed-carrier dials.
pub fn ft8_band_plan() -> Vec<BandChannel> {
    let n = "standard FT8 calling frequency (WSJT-X default)";
    vec![
        ch("160m", "HF", 1.840, "USB", "160 m · FT8", n),
        ch("80m", "HF", 3.573, "USB", "80 m · FT8", n),
        ch(
            "60m",
            "HF",
            5.3715,
            "USB",
            "60 m · FT8",
            "US 60 m channel at 5373.0 kHz centre (dial = centre - 1.5 kHz); 60 m is channelised \
             and the channels differ by country - check your own band plan",
        ),
        ch("40m", "HF", 7.074, "USB", "40 m · FT8", n),
        ch("30m", "HF", 10.136, "USB", "30 m · FT8", n),
        ch("20m", "HF", 14.074, "USB", "20 m · FT8", n),
        ch("17m", "HF", 18.100, "USB", "17 m · FT8", n),
        ch("15m", "HF", 21.074, "USB", "15 m · FT8", n),
        ch("12m", "HF", 24.915, "USB", "12 m · FT8", n),
        ch("10m", "HF", 28.074, "USB", "10 m · FT8", n),
        ch("6m", "VHF", 50.313, "USB", "6 m · FT8", n),
        ch("4m", "VHF", 70.154, "USB", "4 m · FT8", "standard FT8 calling frequency (WSJT-X default) — IARU Region 1 only, no US allocation; 4 m band edges vary widely by country, confirm yours before transmitting"),
        ch("2m", "VHF", 144.174, "USB", "2 m · FT8", n),
        ch("70cm", "UHF", 432.065, "USB", "70 cm · FT8", n),
        ch("23cm", "UHF", 1296.174, "USB", "23 cm · FT8", n),
    ]
}

pub fn ft4_band_plan() -> Vec<BandChannel> {
    let n = "standard FT4 calling frequency (WSJT-X default)";
    vec![
        ch("80m", "HF", 3.575, "USB", "80 m · FT4", n),
        ch("40m", "HF", 7.0475, "USB", "40 m · FT4", n),
        ch("30m", "HF", 10.140, "USB", "30 m · FT4", n),
        ch("20m", "HF", 14.080, "USB", "20 m · FT4", n),
        ch("17m", "HF", 18.104, "USB", "17 m · FT4", n),
        ch("15m", "HF", 21.140, "USB", "15 m · FT4", n),
        ch("12m", "HF", 24.919, "USB", "12 m · FT4", n),
        ch("10m", "HF", 28.180, "USB", "10 m · FT4", n),
        ch("6m", "VHF", 50.318, "USB", "6 m · FT4", n),
        ch("2m", "VHF", 144.170, "USB", "2 m · FT4", n),
    ]
}

pub fn rtty_band_plan() -> Vec<BandChannel> {
    vec![
        ch("160m", "HF", 1.838, "LSB", "160 m · RTTY", "RTTY is rare here; shared with PSK31 1.838 — listen first"),
        ch("80m", "HF", 3.590, "LSB", "80 m · RTTY", "the classic 3.580–3.600 RTTY window; 3.585/3.590 are the DX calling spots — 3.580 itself is PSK31's home"),
        ch("40m", "HF", 7.080, "LSB", "40 m · RTTY (US)", "US activity 7.080–7.100 (ARRL RTTY/data 7.080–7.125)"),
        ch("40m-dx", "HF", 7.045, "LSB", "40 m · RTTY (EU/DX)", "IARU R1 digimode window 7.040–7.047, DX calling 7.040/7.045; WSPR sits at 7.0386 — stay high"),
        ch("30m", "HF", 10.142, "LSB", "30 m · RTTY", "10.140–10.150 data half; secondary band — tread lightly"),
        ch("20m", "HF", 14.083, "LSB", "20 m · RTTY", "the 14.080–14.090 RTTY window, above the FT4 cluster at 14.080"),
        ch("17m", "HF", 18.105, "LSB", "17 m · RTTY", "18.100–18.108 window, above FT8 18.100's audio cluster"),
        ch("15m", "HF", 21.083, "LSB", "15 m · RTTY", "the 21.080–21.100 RTTY window, above JS8 21.078"),
        ch("12m", "HF", 24.920, "LSB", "12 m · RTTY", "the 24.920–24.925 RTTY window (IARU digimodes 24.915–24.929), clear of FT8 24.915"),
        ch("10m", "HF", 28.083, "LSB", "10 m · RTTY", "the 28.080–28.100 RTTY window — Technician-accessible"),
    ]
}

pub fn psk_band_plan() -> Vec<BandChannel> {
    vec![
        ch("160m", "HF", 1.838, "USB", "160 m · PSK31", "the 160 m digimode hole — shared with RTTY and JT65 1.838; listen first"),
        ch("80m", "HF", 3.580, "USB", "80 m · PSK31", "THE 80 m PSK31 home (this file's RTTY plan keeps clear of it at 3.590)"),
        ch("40m", "HF", 7.070, "USB", "40 m · PSK31 (US)", "the classic US/R2 40 m PSK31 watering hole 7.070–7.072"),
        ch("40m-dx", "HF", 7.040, "USB", "40 m · PSK31 (EU/DX)", "IARU R1 digimode window 7.040–7.047 — WSPR sits at 7.0386, stay high; same US/EU split as the RTTY plan's 7.080/7.045"),
        ch("30m", "HF", 10.141, "USB", "30 m · PSK31", "the 10.140–10.142 PSK cluster (the native plan's note cites PSK 10.141); secondary band — tread lightly"),
        ch("20m", "HF", 14.070, "USB", "20 m · PSK31", "THE worldwide PSK31 watering hole, 14.070–14.072 — just below the FT8 cluster at 14.074"),
        ch("17m", "HF", 18.097, "USB", "17 m · PSK31", "the historical 17 m PSK31 spot moved BELOW FT8: 18.100's FT8 audio cluster now owns 18.100–18.103, so activity sits ~18.097–18.099 — listen first"),
        ch("15m", "HF", 21.070, "USB", "15 m · PSK31", "the classic 15 m PSK31 hole 21.070–21.072, below FT8 21.074"),
        ch("12m", "HF", 24.920, "USB", "12 m · PSK31", "the 24.920 digimode spot (IARU digimodes 24.915–24.929), clear of FT8 24.915 — shared with the RTTY window"),
        ch("10m", "HF", 28.120, "USB", "10 m · PSK31", "the classic 10 m PSK31 watering hole — Technician-accessible (10 m data 28.0–28.3)"),
        ch("6m", "VHF", 50.290, "USB", "6 m · PSK31", "the conventional 6 m PSK31 spot, just below WSPR 50.293; activity follows sporadic-E openings — Tech-OK"),
    ]
}

pub fn sstv_band_plan() -> Vec<BandChannel> {
    vec![
        ch("160m", "HF", 1.890, "LSB", "160 m · SSTV", "the conventional 160 m image frequency — rare, and a winter-night band"),
        ch("80m", "HF", 3.845, "LSB", "80 m · SSTV (US)", "NA SSTV calling frequency"),
        ch("80m-eu", "HF", 3.730, "LSB", "80 m · SSTV (EU)", "EU SSTV calling (IARU R1 image centre 3.735) — below US General phone; Extra-class or DX"),
        ch("40m", "HF", 7.171, "LSB", "40 m · SSTV (US)", "US SSTV calling frequency"),
        ch("40m-eu", "HF", 7.165, "LSB", "40 m · SSTV (EU)", "EU/IARU R1 SSTV calling"),
        ch("20m", "HF", 14.230, "USB", "20 m · SSTV", "THE worldwide SSTV calling frequency — where images actually appear"),
        ch("20m-alt", "HF", 14.233, "USB", "20 m · SSTV (alt)", "the first overflow when 14.230 is busy"),
        ch("20m-alt2", "HF", 14.236, "USB", "20 m · SSTV (alt 2)", "the second overflow — commonly used during contests and nets"),
        ch("17m", "HF", 18.160, "USB", "17 m · SSTV", "the conventional 17 m image frequency"),
        ch("15m", "HF", 21.340, "USB", "15 m · SSTV", "worldwide 15 m SSTV calling"),
        ch("12m", "HF", 24.975, "USB", "12 m · SSTV", "the conventional 12 m image frequency"),
        ch("10m", "HF", 28.680, "USB", "10 m · SSTV", "worldwide 10 m SSTV calling (General and above — Technicians have 10 m image only 28.300–28.500)"),
        ch("6m", "VHF", 50.680, "USB", "6 m · SSTV", "the 6 m image frequency; activity follows sporadic-E openings"),
        ch("2m", "VHF", 145.800, "FM", "2 m · ISS downlink", "ARISS events transmit PD120 images here — the SSTV event of the year, FM"),
        ch("2m-call", "VHF", 144.500, "FM", "2 m · SSTV calling", "terrestrial VHF SSTV calling (regional conventions vary — check locally)"),
    ]
}

pub fn band_plan_for(tier: crate::dto::Tier) -> Vec<BandChannel> {
    use crate::dto::Tier;
    match tier {
        Tier::Ft8 => ft8_band_plan(),
        Tier::Ft4 => ft4_band_plan(),
        Tier::Ft2 => ft2_band_plan(),
        Tier::Q65 => q65_band_plan(),
        Tier::Msk144 => msk144_band_plan(),
        Tier::Fst4 => fst4_band_plan(),
        Tier::Fst4w => fst4w_band_plan(),
        Tier::Jt65 => jt65_band_plan(),
        Tier::Wspr => wspr_band_plan(),
        Tier::TempoFast | Tier::TempoDeep => band_plan(),
    }
}

pub fn ft2_band_plan() -> Vec<BandChannel> {
    let n =
        "Decodium FT2 calling frequency (from Decodium's default table, models/FrequencyList.cpp)";
    vec![
        ch("160m", "HF", 1.843000, "USB", "160 m · FT2", n),
        ch("80m", "HF", 3.578000, "USB", "80 m · FT2", n),
        ch(
            "60m",
            "HF",
            5.360000,
            "USB",
            "60 m · FT2",
            "Decodium FT2 calling frequency – check your own band plan",
        ),
        ch("40m", "HF", 7.062000, "USB", "40 m · FT2", n),
        ch(
            "4m",
            "VHF",
            70.157000,
            "USB",
            "4 m · FT2",
            "Decodium FT2 calling frequency – IARU Region 1 only",
        ),
        ch("2m", "VHF", 144.177000, "USB", "2 m · FT2", n),
        ch(
            "1.25m",
            "VHF",
            222.177000,
            "USB",
            "1.25 m · FT2",
            "Decodium FT2 calling frequency – IARU Region 2 only",
        ),
        ch("70cm", "UHF", 432.177000, "USB", "70cm · FT2", n),
        ch("23cm", "UHF", 1296.177000, "USB", "23cm · FT2", n),
    ]
}

pub fn q65_band_plan() -> Vec<BandChannel> {
    let n = "WSJT-X Q65 calling frequency";
    vec![
        ch("6m", "VHF", 50.211000, "USB", "6 m · Q65", n),
        ch("6m-2", "VHF", 50.275000, "USB", "6 m · Q65", n),
        ch("2m", "VHF", 144.116000, "USB", "2 m · Q65", n),
        ch("1.25m", "VHF", 222.065000, "USB", "1.25 m · Q65", n),
        ch("70cm", "VHF", 432.065000, "USB", "70cm · Q65", n),
        ch("33cm", "VHF", 902.065000, "USB", "33cm · Q65", n),
        ch("23cm", "VHF", 1296.065000, "USB", "23cm · Q65", n),
        ch("13cm", "VHF", 2301.065000, "USB", "13cm · Q65", n),
        ch("13cm-2", "VHF", 2304.065000, "USB", "13cm-2 · Q65", n),
        ch("13cm-3", "VHF", 2320.065000, "USB", "13cm-3 · Q65", n),
        ch("9cm", "VHF", 3400.065000, "USB", "9cm · Q65", n),
        ch("6cm", "VHF", 5760.200000, "USB", "6cm · Q65", n),
        ch("3cm", "VHF", 10368.200000, "USB", "3cm · Q65", n),
        ch("1.25cm", "VHF", 24048.200000, "USB", "1.25cm · Q65", n),
    ]
}

pub fn msk144_band_plan() -> Vec<BandChannel> {
    let n = "WSJT-X MSK144 calling frequency";
    vec![
        ch("6m", "VHF", 50.260000, "USB", "6 m · MSK144", n),
        ch("6m-2", "VHF", 50.380000, "USB", "6 m · MSK144", n),
        ch("4m", "VHF", 70.230000, "USB", "4 m · MSK144", n),
        ch("2m", "VHF", 144.150000, "USB", "2 m · MSK144", n),
        ch("2m-2", "VHF", 144.360000, "USB", "2 m · MSK144", n),
        ch("70cm", "VHF", 432.360000, "USB", "70cm · MSK144", n),
    ]
}

pub fn fst4_band_plan() -> Vec<BandChannel> {
    let n = "WSJT-X FST4 calling frequency";
    vec![
        ch("2200m", "HF", 0.136000, "USB", "2200 m · FST4", n),
        ch("630m", "HF", 0.474200, "USB", "630 m · FST4", n),
        ch("160m", "HF", 1.839000, "USB", "160 m · FST4", n),
    ]
}

pub fn fst4w_band_plan() -> Vec<BandChannel> {
    let n = "WSJT-X FST4W beacon frequency";
    vec![
        ch("2200m", "HF", 0.136000, "USB", "2200 m · FST4W", n),
        ch("630m", "HF", 0.474200, "USB", "630 m · FST4W", n),
        ch("160m", "HF", 1.836800, "USB", "160 m · FST4W", n),
    ]
}

pub fn jt65_band_plan() -> Vec<BandChannel> {
    let n = "WSJT-X JT65 calling frequency";
    vec![
        ch("160m", "HF", 1.838000, "USB", "160 m · JT65", n),
        ch("80m", "HF", 3.570000, "USB", "80 m · JT65", n),
        ch("40m", "HF", 7.076000, "USB", "40 m · JT65", n),
        ch("30m", "HF", 10.138000, "USB", "30 m · JT65", n),
        ch("20m", "HF", 14.076000, "USB", "20 m · JT65", n),
        ch("17m", "HF", 18.102000, "USB", "17 m · JT65", n),
        ch("15m", "HF", 21.076000, "USB", "15 m · JT65", n),
        ch("12m", "HF", 24.917000, "USB", "12 m · JT65", n),
        ch("10m", "HF", 28.076000, "USB", "10 m · JT65", n),
        ch("6m", "VHF", 50.276000, "USB", "6 m · JT65", n),
        ch("6m-2", "VHF", 50.310000, "USB", "6 m · JT65", n),
        ch("4m", "VHF", 70.102000, "USB", "4 m · JT65", n),
        ch("2m", "VHF", 144.120000, "USB", "2 m · JT65", n),
        ch("1.25m", "VHF", 222.065000, "USB", "1.25 m · JT65", n),
        ch("70cm", "VHF", 432.065000, "USB", "70cm · JT65", n),
        ch("33cm", "VHF", 902.065000, "USB", "33cm · JT65", n),
        ch("23cm", "VHF", 1296.065000, "USB", "23cm · JT65", n),
        ch("13cm", "VHF", 2301.065000, "USB", "13cm · JT65", n),
        ch("13cm-2", "VHF", 2304.065000, "USB", "13cm-2 · JT65", n),
        ch("13cm-3", "VHF", 2320.065000, "USB", "13cm-3 · JT65", n),
        ch("9cm", "VHF", 3400.065000, "USB", "9cm · JT65", n),
        ch("6cm", "VHF", 5760.065000, "USB", "6cm · JT65", n),
    ]
}

pub fn wspr_band_plan() -> Vec<BandChannel> {
    let n = "WSPR beacon frequency";
    vec![
        ch("2200m", "HF", 0.136000, "USB", "2200 m · WSPR", n),
        ch("630m", "HF", 0.474200, "USB", "630 m · WSPR", n),
        ch("160m", "HF", 1.836600, "USB", "160 m · WSPR", n),
        ch("80m", "HF", 3.568600, "USB", "80 m · WSPR", n),
        ch("40m", "HF", 7.038600, "USB", "40 m · WSPR", n),
        ch("30m", "HF", 10.138700, "USB", "30 m · WSPR", n),
        ch("20m", "HF", 14.095600, "USB", "20 m · WSPR", n),
        ch("17m", "HF", 18.104600, "USB", "17 m · WSPR", n),
        ch("15m", "HF", 21.094600, "USB", "15 m · WSPR", n),
        ch("12m", "HF", 24.924600, "USB", "12 m · WSPR", n),
        ch("10m", "HF", 28.124600, "USB", "10 m · WSPR", n),
        ch("6m", "VHF", 50.293000, "USB", "6 m · WSPR", n),
        ch("4m", "VHF", 70.091000, "USB", "4 m · WSPR", n),
        ch("2m", "VHF", 144.489000, "USB", "2 m · WSPR", n),
        ch("70cm", "VHF", 432.300000, "USB", "70cm · WSPR", n),
        ch("23cm", "VHF", 1296.500000, "USB", "23cm · WSPR", n),
    ]
}

pub fn cw_activity_mhz(band: &str) -> Option<f64> {
    Some(match band {
        "160m" => 1.810,
        "80m" => 3.550,
        "40m" => 7.030,
        "30m" => 10.110,
        "20m" => 14.030,
        "17m" => 18.080,
        "15m" => 21.030,
        "12m" => 24.900,
        "11m" => 27.555,
        "10m" => 28.030,
        "6m" => 50.090,
        "4m" => 70.200,
        "2m" => 144.050,
        "1.25m" => 222.050,
        "70cm" => 432.050,
        "23cm" => 1296.050,
        _ => return None,
    })
}

pub fn channel_for_dial(dial_mhz: f64) -> Option<BandChannel> {
    band_plan()
        .into_iter()
        .find(|c| (c.dial_mhz - dial_mhz).abs() < 0.0005)
}

pub fn band_for_dial(dial_mhz: f64) -> Option<&'static str> {
    let b = match dial_mhz {
        f if (1.8..2.0).contains(&f) => "160m",
        f if (3.5..4.0).contains(&f) => "80m",
        f if (5.3..5.5).contains(&f) => "60m",
        f if (7.0..7.3).contains(&f) => "40m",
        f if (10.1..10.15).contains(&f) => "30m",
        f if (14.0..14.35).contains(&f) => "20m",
        f if (18.06..18.17).contains(&f) => "17m",
        f if (21.0..21.45).contains(&f) => "15m",
        f if (24.89..24.99).contains(&f) => "12m",
        f if (26.0..28.0).contains(&f) => "11m",
        f if (28.0..29.7).contains(&f) => "10m",
        f if (50.0..54.0).contains(&f) => "6m",
        f if (70.0..71.0).contains(&f) => "4m",
        f if (144.0..148.0).contains(&f) => "2m",
        f if (222.0..225.0).contains(&f) => "1.25m",
        f if (420.0..450.0).contains(&f) => "70cm",
        f if (902.0..928.0).contains(&f) => "33cm",
        f if (1240.0..1300.0).contains(&f) => "23cm",
        f if (2300.0..2450.0).contains(&f) => "13cm",
        f if (3300.0..3500.0).contains(&f) => "9cm",
        f if (5650.0..5925.0).contains(&f) => "6cm",
        f if (10_000.0..10_500.0).contains(&f) => "3cm",
        f if (24_000.0..24_250.0).contains(&f) => "1.25cm",
        _ => return None,
    };
    Some(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_4m_band_exists_end_to_end() {
        use crate::dto::Tier;
        let dial = |t: Tier| -> Option<f64> {
            band_plan_for(t)
                .into_iter()
                .find(|c| canonical_band(&c.band) == "4m")
                .map(|c| c.dial_mhz)
        };
        assert_eq!(dial(Tier::Ft8), Some(70.154));
        assert_eq!(dial(Tier::Jt65), Some(70.102));
        assert_eq!(dial(Tier::Msk144), Some(70.230));
        assert_eq!(dial(Tier::Wspr), Some(70.091));
        assert_eq!(dial(Tier::Ft4), None);
        assert_eq!(dial(Tier::Q65), None);
        for f in [70.091, 70.102, 70.154, 70.230] {
            assert_eq!(super::band_for_dial(f), Some("4m"), "{} MHz is 4 m", f);
        }
    }

    #[test]
    fn plan_is_nonempty_and_well_formed() {
        let plan = band_plan();
        assert!(plan.len() >= 14);
        let b11 = plan.iter().find(|c| c.band == "11m").expect("11m present");
        assert!(b11.tx);
    }

    #[test]
    fn cw_activity_is_inside_band_and_off_the_edge() {
        use crate::privileges::tx_allowed;
        use crate::settings::{LicenseClass, OperatingMode};
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
