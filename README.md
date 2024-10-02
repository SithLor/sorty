# sorty
library for sort array safely slower hardware


pub fn expand_file_path(input: String) -> Vec<String> {
    let mut output = Vec::new();

    // Find the positions of '{' and '}'
    let list_start_position = input.find('{').expect("No '{' found in input");
    let list_end_position = input.find('}').expect("No '}' found in input");

    // Extract the base path and the list of filenames
    let base_path = &input[..list_start_position];
    let filenames = &input[list_start_position + 1..list_end_position];
    let suffix = &input[list_end_position + 1..];

    // Split the filenames by ',' and construct the full paths
    for filename in filenames.split(',') {
        output.push(format!("{}{}{}", base_path, filename, suffix));
    }

    output
}

fn main() {
    let input = String::from("/path/to/{file1,file2,file3}.txt");
    let expanded_paths = expand_file_path(input);
    for path in expanded_paths {
        println!("{}", path);
    }
}