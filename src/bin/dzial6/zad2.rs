//Napisz funkcję o nagłówku
//fn wartosc_syst2(z: &str) -> Option<u8>
//obliczającą wartość zapisaną w systemie dwójkowym — pod warunkiem,
//że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak
//inny niż cyfra dwójkowa lub parametr jest pusty), to wynikiem jest None.

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

fn main() {
    wartosc_syst2("1");
}

#[test]
fn test_wartosc_syst2() {
    assert_eq!(wartosc_syst2("011111111101"), None);
    assert_eq!(wartosc_syst2("11111111"), Some(255));
    assert_eq!(wartosc_syst2(""), None);
    assert_eq!(wartosc_syst2("0111s1111"), None);   
}
