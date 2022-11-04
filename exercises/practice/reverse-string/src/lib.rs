#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    let mut new_string = String::new();
    for var in input.chars().rev(){
        new_string.push(var);
    }

    //return input.chars().rev().collect();

    println!("Original string: {}, reversed string: {}", input, new_string);
    return new_string;
}
#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    let mut new_string = String::new();

    for var in input.graphemes(true).rev(){
        new_string.push_str(var);
    }


    //return input.chars().rev().collect();
    println!("Original string: {}, reversed string: {}", input, new_string);
    return new_string;
}

