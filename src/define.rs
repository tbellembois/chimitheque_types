use regex::Regex;

pub static PRECAUTIONARY_STATEMENT_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| {
        Regex::new(r"(?P<reference>P[0-9+]+)(\t)(?P<label>[^\t]+)").unwrap()
    });
pub static HAZARD_STATEMENT_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"(?P<reference>(EU){0,1}H[0-9]+)(\t)(?P<label>[^\t]+)(\t)").unwrap()
});
pub static SYMBOL_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"(?P<symbol>GHS0[1-9])").unwrap());

pub static IDS_MATCH_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"^((\d+),{0,1})+$").unwrap());
pub static IDS_CAPTURE_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"(?<id>\d+),{0,1}").unwrap());
pub static END_OF_URL_CAPTURE_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"/(?<id>\d+)$").unwrap());
