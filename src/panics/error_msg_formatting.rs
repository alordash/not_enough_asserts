const EXPECTED_SECTION_HEADER_TEXT: &'static str = " Expected ";
const ACTUAL_SECTION_HEADER_TEXT: &'static str = " Actual ";
const SECTION_SEPARATOR_CHAR: char = '—';

pub(crate) fn format_error_msg(expected_error_msg: &str, actual_error_msg: String) -> String {
    let maximum_msg_line_length = get_maximum_string_line_length(&expected_error_msg)
        .max(get_maximum_string_line_length(&actual_error_msg));

    let expected_section_header = surround_string_with_char(
        EXPECTED_SECTION_HEADER_TEXT,
        SECTION_SEPARATOR_CHAR,
        maximum_msg_line_length,
    );
    let actual_section_header = surround_string_with_char(
        ACTUAL_SECTION_HEADER_TEXT,
        SECTION_SEPARATOR_CHAR,
        maximum_msg_line_length,
    );
    let last_separator: String = core::iter::repeat(SECTION_SEPARATOR_CHAR)
        .take(maximum_msg_line_length)
        .collect();
    let result = format!(
        "Wrong panic message:
{expected_section_header}
{expected_error_msg}
{actual_section_header}
{actual_error_msg}
{last_separator}"
    );
    return result;
}

fn get_maximum_string_line_length(string: &str) -> usize {
    string.lines().map(str::len).max().unwrap_or(0)
}

fn surround_string_with_char(str: &str, char: char, result_string_length: usize) -> String {
    let Some(chars_count) = result_string_length.checked_sub(str.len()) else {
        return str.to_string();
    };
    let right_chars_length = chars_count / 2;
    let left_chars_length = chars_count - right_chars_length;
    let left_chars: String = core::iter::repeat(char).take(left_chars_length).collect();
    let right_chars: String = core::iter::repeat(char).take(right_chars_length).collect();
    return left_chars + str + &right_chars;
}
