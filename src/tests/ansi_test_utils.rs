#[cfg(test)]
pub mod ansi_test_utils {
    use ansi_term;
    use console::strip_ansi_codes;

    use crate::config::Config;
    use crate::paint;
    use crate::style::Style;

    pub fn assert_line_has_style(
        output: &str,
        line_number: usize,
        expected_prefix: &str,
        expected_style: &str,
        config: &Config,
    ) {
        assert!(_line_has_style(
            output,
            line_number,
            expected_prefix,
            expected_style,
            config,
            false,
        ));
    }

    pub fn assert_line_does_not_have_style(
        output: &str,
        line_number: usize,
        expected_prefix: &str,
        expected_style: &str,
        config: &Config,
    ) {
        assert!(!_line_has_style(
            output,
            line_number,
            expected_prefix,
            expected_style,
            config,
            false,
        ));
    }

    pub fn assert_line_has_4_bit_color_style(
        output: &str,
        line_number: usize,
        expected_prefix: &str,
        expected_style: &str,
        config: &Config,
    ) {
        assert!(_line_has_style(
            output,
            line_number,
            expected_prefix,
            expected_style,
            config,
            true,
        ));
    }

    pub fn assert_line_has_no_color(output: &str, line_number: usize, expected_prefix: &str) {
        let line = output.lines().nth(line_number).unwrap();
        let stripped_line = strip_ansi_codes(line);
        assert!(stripped_line.starts_with(expected_prefix));
        assert_eq!(line, stripped_line);
    }

    /// Assert that the specified line number of output (a) matches
    /// `expected_prefix` and (b) for the length of expected_prefix is
    /// syntax-highlighted according to `language_extension`.
    pub fn assert_line_is_syntax_highlighted(
        output: &str,
        line_number: usize,
        expected_prefix: &str,
        language_extension: &str,
        config: &Config,
    ) {
        let line = output.lines().nth(line_number).unwrap();
        let stripped_line = &strip_ansi_codes(line);
        assert!(stripped_line.starts_with(expected_prefix));
        let painted_line = paint_line(expected_prefix, language_extension, config);
        // remove trailing newline appended by paint::paint_lines.
        assert!(line.starts_with(painted_line.trim_end()));
    }

    pub fn assert_has_color_other_than_plus_color(string: &str, config: &Config) {
        let (string_without_any_color, string_with_plus_color_only) =
            get_color_variants(string, config);
        assert_ne!(string, string_without_any_color);
        assert_ne!(string, string_with_plus_color_only);
    }

    pub fn assert_has_plus_color_only(string: &str, config: &Config) {
        let (string_without_any_color, string_with_plus_color_only) =
            get_color_variants(string, config);
        assert_ne!(string, string_without_any_color);
        assert_eq!(string, string_with_plus_color_only);
    }

    pub fn get_color_variants(string: &str, config: &Config) -> (String, String) {
        let string_without_any_color = strip_ansi_codes(string).to_string();
        let string_with_plus_color_only = config
            .plus_style
            .ansi_term_style
            .paint(&string_without_any_color);
        (
            string_without_any_color.to_string(),
            string_with_plus_color_only.to_string(),
        )
    }

    pub fn paint_line(line: &str, language_extension: &str, config: &Config) -> String {
        let mut output_buffer = String::new();
        let mut unused_writer = Vec::<u8>::new();
        let mut painter = paint::Painter::new(&mut unused_writer, config);
        let syntax_highlighted_style = Style {
            is_syntax_highlighted: true,
            ..Style::new()
        };
        painter.set_syntax(Some(language_extension));
        painter.set_highlighter();
        let line = format!(" {}", line); // TODO: a leading space must be added, as delta::prepare() does
        let lines = vec![&line];
        let syntax_style_sections = painter.highlighter.highlight(&line, &config.syntax_set);
        paint::Painter::paint_lines(
            vec![syntax_style_sections],
            vec![vec![(syntax_highlighted_style, lines[0])]],
            vec![None],
            &mut output_buffer,
            config,
            "",
            config.null_style,
            config.null_style,
            None,
            None,
        );
        output_buffer
    }

    fn _line_has_style(
        output: &str,
        line_number: usize,
        expected_prefix: &str,
        expected_style: &str,
        config: &Config,
        _4_bit_color: bool,
    ) -> bool {
        let line = output.lines().nth(line_number).unwrap();
        assert!(strip_ansi_codes(line).starts_with(expected_prefix));
        let mut style = Style::from_str(expected_style, None, None, config.true_color, false);
        if _4_bit_color {
            style.ansi_term_style.foreground = style
                .ansi_term_style
                .foreground
                .map(ansi_term_fixed_foreground_to_4_bit_color);
        }
        line.starts_with(&style.ansi_term_style.prefix().to_string())
    }

    fn ansi_term_fixed_foreground_to_4_bit_color(color: ansi_term::Color) -> ansi_term::Color {
        match color {
            ansi_term::Color::Fixed(30) => ansi_term::Color::Black,
            ansi_term::Color::Fixed(31) => ansi_term::Color::Red,
            ansi_term::Color::Fixed(32) => ansi_term::Color::Green,
            ansi_term::Color::Fixed(33) => ansi_term::Color::Yellow,
            ansi_term::Color::Fixed(34) => ansi_term::Color::Blue,
            ansi_term::Color::Fixed(35) => ansi_term::Color::Purple,
            ansi_term::Color::Fixed(36) => ansi_term::Color::Cyan,
            ansi_term::Color::Fixed(37) => ansi_term::Color::White,
            color => panic!(
                "Invalid 4-bit color: {:?}. \
                 (Add bright color entries to this map if needed for tests.",
                color
            ),
        }
    }
}
