pub const TEAM_COLORS: [&str; 32] = [
    "#696969", "#556B2F", "#8B4513", "#228B22", "#483D8B", "#B8860B", "#008B8B", "#4682B4",
    "#000080", "#32CD32", "#7F007F", "#8FBC8F", "#B03060", "#FF0000", "#FF8C00", "#FFFF00",
    "#00FF00", "#9400D3", "#00FA9A", "#DC143C", "#00FFFF", "#0000FF", "#ADFF2F", "#FF00FF",
    "#1E90FF", "#F0E68C", "#FA8072", "#ADD8E6", "#FF1493", "#7B68EE", "#EE82EE", "#FFB6C1",
];

pub fn is_valid_hex(color: &str) -> bool {
    if color.len() != 7 || !color.starts_with('#') {
        return false;
    }
    color[1..].chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_hex() {
        assert!(is_valid_hex("#FFFFFF"));
        assert!(is_valid_hex("#000000"));
        assert!(!is_valid_hex("#GGGGGG")); // Invalid hex
        assert!(!is_valid_hex("FFFFFF")); // Missing '#'
    }

    #[test]
    fn test_team_colors() {
        for color in TEAM_COLORS.iter() {
            assert!(is_valid_hex(color), "Invalid hex color: {color}");
        }
    }
}
