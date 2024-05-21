// Napisz funkcję o nagłówku
// fn zamien_syst8_na_syst2(z: &str) -> Option<String>
// zamieniającą zapis liczby całkowitej w systemie ósemkowym na zapis w systemie dwójkowym. Wynik ma być najkrótszy możliwy, niepusty.
//Wynik None ma oznaczać wystąpienie w
//parametrze z niedozwolonego znaku (spoza cyfr ósemkowych) lub pusty napis w parametrze.

fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.is_empty() {
        return None;
    }
    let mut result = String::new();
    for c in z.chars() {
        match c {
            '0' => result.push_str("000"),
            '1' => result.push_str("001"),
            '2' => result.push_str("010"),
            '3' => result.push_str("011"),
            '4' => result.push_str("100"),
            '5' => result.push_str("101"),
            '6' => result.push_str("110"),
            '7' => result.push_str("111"),
            _ => break,
        }
    }
    result = result.trim_start_matches("0").to_string();
    return Some(result);
}
fn main() {
    println!(
        "{:?}",
        assert_eq!(zamien_syst8_na_syst2("16").unwrap(), "1110")
    );
}
#[test]
fn test_zamien_syst8_na2() {
    assert_eq!(zamien_syst8_na_syst2("16").unwrap(), "1110")
}
