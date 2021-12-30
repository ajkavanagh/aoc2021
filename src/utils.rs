// handy utilities


use std::str::FromStr;


/// Read lines from a file and parse them into a vector; blows up if the file is not found.
/// The vector is a vector of Results.
/// Use like
///     let things = read_file::<Thing>("filename");
pub fn read_file<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}



/// process a set of lines into a <Vec<Vec<&str>> -- i.e. don't copy the lines, just their
/// references.  We split batches on blank lines.
pub fn process_lines_to_batches<'a>(lines: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut result = Vec::new();
    let mut batch = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            if !batch.is_empty() {
                result.push(batch.clone());
                batch.clear();
            }
        } else {
            batch.push(*line);
        }
    }
    if !batch.is_empty() {
        result.push(batch.clone());
    }
    result
}
