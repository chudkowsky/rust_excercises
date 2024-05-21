fn wartosc_cyfry(c: char) -> Result<u8, String>{
    match c{
        '0' => return Ok(0),
        '1' => return Ok(1),
        '2' => return Ok(2),
        '3' => return Ok(3),
        '4' => return Ok(4),
        '5' => return Ok(5),
        '6' => return Ok(6),
        '7' => return Ok(7),
        '8' => return Ok(8),
        '9' => return Ok(9),
        _ => return Err("Podales zly znak".to_string()),
    }
}

fn main() {
    println!("{:?}", wartosc_cyfry('1'));
    println!("{:?}", wartosc_cyfry('2'));
    println!("{:?}", wartosc_cyfry('f'));
}