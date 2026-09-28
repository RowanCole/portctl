use crate::i18n;

// 全角字符按 2 列计算显示宽度
fn disp_width(s: &str) -> usize {
    s.chars()
        .map(|c| {
            let cp = c as u32;
            let wide = (0x1100..=0x115F).contains(&cp)
                || (0x2E80..=0xA4CF).contains(&cp)
                || (0xAC00..=0xD7A3).contains(&cp)
                || (0xF900..=0xFAFF).contains(&cp)
                || (0xFE30..=0xFE4F).contains(&cp)
                || (0xFF00..=0xFF60).contains(&cp)
                || (0xFFE0..=0xFFE6).contains(&cp)
                || cp >= 0x20000;
            if wide {
                2
            } else {
                1
            }
        })
        .sum()
}

pub fn print_table(rows: &[Vec<String>]) {
    let headers = i18n::HEADERS;
    let mut widths: Vec<usize> = headers.iter().map(|h| disp_width(h)).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(disp_width(cell));
        }
    }

    let print_line = |cells: &[String]| {
        let mut line = String::new();
        for (i, cell) in cells.iter().enumerate() {
            line.push_str(cell);
            line.push_str(&" ".repeat(widths[i] - disp_width(cell) + 2));
        }
        println!("{}", line.trim_end());
    };

    print_line(&headers.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    println!(
        "{}",
        widths
            .iter()
            .map(|w| "-".repeat(*w))
            .collect::<Vec<_>>()
            .join("  ")
    );
    for row in rows {
        print_line(row);
    }
}
