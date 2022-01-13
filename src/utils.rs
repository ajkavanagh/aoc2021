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


pub fn read_file_single_result<T>(file_name: &str) -> Result<Vec<T>, <T as FromStr>::Err>
    where T: FromStr + Clone,
          <T as FromStr>::Err: Clone,
{
    read_file::<T>(file_name).iter().cloned().collect::<Result<Vec<_>, _>>()
}

/// process a set of lines into a <Vec<Vec<&str>> -- i.e. don't copy the lines, just their
/// references.  We split batches on blank lines.
//pub fn process_lines_to_batches<'a>(lines: &[&'a str]) -> Vec<Vec<&'a str>> {
    //let mut result = Vec::new();
    //let mut batch = Vec::new();
    //for line in lines {
        //if line.trim().is_empty() {
            //if !batch.is_empty() {
                //result.push(batch.clone());
                //batch.clear();
            //}
        //} else {
            //batch.push(*line);
        //}
    //}
    //if !batch.is_empty() {
        //result.push(batch.clone());
    //}
    //result
//}


//pub fn sort_string<S: ToString>(s: S) -> String {
    //let mut cs = s.to_string().chars().collect::<Vec<char>>();
    //cs.sort_by(|a,b| a.cmp(b));
    //cs.iter().collect()
//}


/// count the number of bits in a
pub fn count_bits(v: usize, n: usize) -> usize
{
    let mut count: usize = 0;
    let mut bit: usize = 1;
    for _ in 0..n {
        if v & bit == bit {
            count +=1;
        }
        bit *= 2;
    }
    count
}

