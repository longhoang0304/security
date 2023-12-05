pub fn take_words_array_start(data: &Vec<u8>, start: usize, number_of_words: usize) -> Vec<u16>
{
    let mut value: Vec<u16> = Vec::new();

    for i in (start..start + number_of_words).step_by(2).rev() {
        let upper: u16 = (data.get(i + 1).unwrap_or(&0).clone() as u16) << 8;
        value.push(upper | data[i] as u16);
    }
    value
}
