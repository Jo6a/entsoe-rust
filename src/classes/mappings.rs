use phf::{phf_map};
pub struct Mappings {
    pub dummy: bool
} 

impl Mappings {
    pub const DOMAIN_MAPPINGS: phf::Map<&'static str, &'static str> = phf_map! {
        "AL" => "10YAL-KESH-----5",
        "AT" => "10YAT-APG------L",
        "BA" => "10YBA-JPCC-----D",
        "BE" => "10YBE----------2",
        "BG" => "10YCA-BULGARIA-R",
        "BY" => "10Y1001A1001A51S",
        "CH" => "10YCH-SWISSGRIDZ",
        "CZ" => "10YCZ-CEPS-----N",
        "DE" => "10Y1001A1001A83F",
        "DK" => "10Y1001A1001A65H",
        "EE" => "10Y1001A1001A39I",
        "ES" => "10YES-REE------0",
        "FI" => "10YFI-1--------U",
        "FR" => "10YFR-RTE------C",
        "GB" => "10YGB----------A",
        "GB_NIR" => "10Y1001A1001A016",
        "GR" => "10YGR-HTSO-----Y",
        "HR" => "10YHR-HEP------M",
        "HU" => "10YHU-MAVIR----U",
        "IE" => "10YIE-1001A00010",
        "IT" => "10YIT-GRTN-----B",
        "LT" => "10YLT-1001A0008Q",
        "LU" => "10YLU-CEGEDEL-NQ",
        "LV" => "10YLV-1001A00074",
        "MD" => "MD",
        "ME" => "10YCS-CG-TSO---S",
        "MK" => "10YMK-MEPSO----8",
        "MT" => "10Y1001A1001A93C",
        "NL" => "10YNL----------L",
        "NO" => "10YNO-0--------C",
        "PL" => "10YPL-AREA-----S",
        "PT" => "10YPT-REN------W",
        "RO" => "10YRO-TEL------P",
        "RS" => "10YCS-SERBIATSOV",
        "RU" => "10Y1001A1001A49F",
        "RU_KGD" => "10Y1001A1001A50U",
        "SE" => "10YSE-1--------K",
        "SI" => "10YSI-ELES-----O",
        "SK" => "10YSK-SEPS-----K",
        "TR" => "10YTR-TEIAS----W",
        "UA" => "10YUA-WEPS-----0",
        "DE_AT_LU" => "10Y1001A1001A63L",
        "DE_LU" => "10Y1001A1001A82H",
    };
}