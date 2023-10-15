/*This is library code that is a Marco Polo Function */

// Marco Polo function that looks for "Marco" and returns "Polo" string
pub fn marco_polo(word: &str) -> String {
    println!("{}", word);
    if word == "Marco" {
        return "Polo".to_string();
    } else {
        return "Not Marco".to_string();
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_marco_polo() {
        assert_eq!(marco_polo("Marco"), "Polo");
        assert_eq!(marco_polo("Not Marco"), "Not Marco");
    }
}
