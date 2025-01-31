

pub fn card_class(string: String) -> String {
    match string.as_str() {
        "HA" => String::from("heart-ace"),
        "H2" => String::from("heart-two"),
        "H3" => String::from("heart-three"),
        "H4" => String::from("heart-four"),
        "H5" => String::from("heart-five"),
        "H6" => String::from("heart-six"),
        "H7" => String::from("heart-seven"),
        "H8" => String::from("heart-eight"),
        "H9" => String::from("heart-nine"),
        "H10" => String::from("heart-ten"),
        "HJ" => String::from("heart-jack"),
        "HQ" => String::from("heart-queen"),
        "HK" => String::from("heart-king"),

        "DA" => String::from("diamond-ace"),
        "D2" => String::from("diamond-two"),
        "D3" => String::from("diamond-three"),
        "D4" => String::from("diamond-four"),
        "D5" => String::from("diamond-five"),
        "D6" => String::from("diamond-six"),
        "D7" => String::from("diamond-seven"),
        "D8" => String::from("diamond-eight"),
        "D9" => String::from("diamond-nine"),
        "D10" => String::from("diamond-ten"),
        "DJ" => String::from("diamond-jack"),
        "DQ" => String::from("diamond-queen"),
        "DK" => String::from("diamond-king"),

        "SA" => String::from("spade-ace"),
        "S2" => String::from("spade-two"),
        "S3" => String::from("spade-three"),
        "S4" => String::from("spade-four"),
        "S5" => String::from("spade-five"),
        "S6" => String::from("spade-six"),
        "S7" => String::from("spade-seven"),
        "S8" => String::from("spade-eight"),
        "S9" => String::from("spade-nine"),
        "S10" => String::from("spade-ten"),
        "SJ" => String::from("spade-jack"),
        "SQ" => String::from("spade-queen"),
        "SK" => String::from("spade-king"),

        "CA" => String::from("club-ace"),
        "C2" => String::from("club-two"),
        "C3" => String::from("club-three"),
        "C4" => String::from("club-four"),
        "C5" => String::from("club-five"),
        "C6" => String::from("club-six"),
        "C7" => String::from("club-seven"),
        "C8" => String::from("club-eight"),
        "C9" => String::from("club-nine"),
        "C10" => String::from("club-ten"),
        "CJ" => String::from("club-jack"),
        "CQ" => String::from("club-queen"),
        "CK" => String::from("club-king"),

        _ => String::from(""),
    }
}