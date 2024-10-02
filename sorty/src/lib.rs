//javascript with extra features


// take "/path/to/{file1,file2,file3}.txt" and expand it to ["/path/to/file1.txt", "/path/to/file2.txt", "/path/to/file3.txt"]
pub fn expand_file_path(input:String) -> Vec<String> {
    use std::str::Chars;
    let mut output = Vec::new();

    let mut chars: Chars<'_> = input.chars();

    //look for the first '{'
    let mut list_start_postion = 0;
    let mut list_end_postion = 0;
    let mut filename_in_list = Vec::new();

    chars.position(|c| {
        if c == '{' {
            list_start_postion = chars.as_str().find('{').unwrap();
            return true;
        }
        false
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_up() {
        let mut v = vec![4, 1, 5, 2, 3];
        sort(&mut v);
        assert_eq!(v, &[1, 2, 3, 4, 5]);
    }
}
