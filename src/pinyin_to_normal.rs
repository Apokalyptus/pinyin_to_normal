use std::collections::HashMap;

pub fn pinyin_to_normal(input_string: String) -> String {
    let pinyin = [
        'ā','á','ǎ','à',
        'ē','é','ě','è',
        'ī','í','ǐ','ì',
        'ō','ó','ǒ','ò',
        'ū','ú','ǔ','ù',
        'ǖ','ǘ','ǚ','ǜ'
    ];
    
    let normal = [
        'a','a','a','a',
        'e','e','e','e',
        'i','i','i','i',
        'o','o','o','o',
        'u','u','u','u',
        'ü','ü','ü','ü'
    ];

    let char_map: HashMap<_, _> = pinyin.iter().zip(normal.iter()).collect();
    let mut result = String::with_capacity(input_string.len());

    for c in input_string.chars() {
        match char_map.get(&c) {
            Some(n) => result.push(**n),
            None => result.push(c),
        }
    }
    
    result
}


#[test]
fn conversion() {
    let test_val = String::from("abcd...z āáǎàēéěèīíǐìōóǒòūúǔùǖǘǚǜ");
    let check_val = String::from("abcd...z aaaaeeeeiiiioooouuuuüüüü");
    assert_eq!(pinyin_to_normal(test_val),check_val);
}
