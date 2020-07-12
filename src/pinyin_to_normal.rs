pub fn pinyin_to_normal(input_string: String) -> String {
    let _pinyin = [ 'ā','á','ǎ','à',
                    'ē','é','ě','è',
                    'ī','í','ǐ','ì',
                    'ō','ó','ǒ','ò',
                    'ū','ú','ǔ','ù',
                    'ǖ','ǘ','ǚ','ǜ'            
                ];

    let _normal = [ 'a','a','a','a',
                    'e','e','e','e',
                    'i','i','i','i',
                    'o','o','o','o',
                    'u','u','u','u',
                    'ü','ü','ü','ü'
                ];

    let mut result = String::new();

    for (_i, _c) in input_string.chars().enumerate(){
        
        let mut tmp = String::new();
     
        for _j in 0.._pinyin.len() {
            if _c == _pinyin[_j] {
                tmp = _normal[_j].to_string();
                continue;
            } 
        }
        if tmp.is_empty() {
            tmp = _c.to_string();
        }
        result = [result,tmp].join("");

        
    }
    return result;
}


#[test]
fn conversion() {
    let test_val = String::from("abcd...z āáǎàēéěèīíǐìōóǒòūúǔùǖǘǚǜ");
    let check_val = String::from("abcd...z aaaaeeeeiiiioooouuuuüüüü");
    assert_eq!(pinyin_to_normal(test_val),check_val);
}
