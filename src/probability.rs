use crate::cursor_consts::{COLOUR_RESET, CYAN, GREEN, RED};

pub fn game_length_chance_percent(win_count: usize, given: usize) -> f64 {
    100.0
        * (given..=win_count)
            .map(|count| given as f64 / count as f64)
            .reduce(|acc, probability| acc * probability)
            .unwrap_or(1.0)
}

pub fn map_range(max_s: f64, rang: (f64, f64), current: f64, scale: Option<f64>) -> (f64, f64) {
    let scale: f64 = match scale {
        Some(s) => s,
        None => 1.0,
    };

    ((rang.0 + current * (rang.1 - rang.0) / max_s) * scale, current)
}

pub fn barz(vals: f64, label: f64) -> String {
    if vals < 0.01 {
        return format!("{label:.2}: Too small to display.").to_string();
    }

    let mut st = format!("{label:.2}");

    match st.chars().count() {
        6 => st.push_str(&format!(":{0}  ", GREEN).to_string()),
        5 => st.push_str(&format!(":{0}   ", CYAN).to_string()),
        4 => st.push_str(&format!(":{0}    ", RED).to_string()),
        _ => st.push_str(""),
    }

    for _ in 0..=vals as usize {
        st.push('█')
    }
    st.push_str(&format!("{0}", COLOUR_RESET).to_string());
    st
}
