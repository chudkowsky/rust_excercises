//Napisz funkcję o nagłówku
//fn wartosc_syst8(z: &str) -> Option<u8>
//obliczającą wartość zapisaną w systemie ósemkowym — pod warunkiem, że mieści się na ośmiu bitach. 
//Jeśli nie (lub w zapisie występuje znak inny niż cyfra ósemkowa lub parametr jest pusty), to wynikiem jest None.
//Uwaga! Funkcję tę należy zbudować z funkcji z zadań poprzednich.

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

fn wartosc_syst2(z: &str) -> Option<u8> {
    if z.len() > 8 {
        return None;
    }
    if z.is_empty() {
        return None;
    }
    let mut power = 0;
    let mut result: u8 = 0;
    for c in z.chars().rev() {
        match c {
            '1' => result += 1 * 2u8.pow(power),
            '0' => result += 0,
             _ => return None,
        }
        power += 1;
    }
    return Some(result);
}
fn wartosc_syst8(z: &str) -> Option<u8>{
    wartosc_syst2(&zamien_syst8_na_syst2(z)?)
}
fn main(){
    wartosc_syst8("2");
}
#[test]
fn test_wartosc_syst8(){
    assert_eq!(wartosc_syst8("16"),Some(14))
}