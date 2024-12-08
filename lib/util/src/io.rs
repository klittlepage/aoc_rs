use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_to_vec(path: &Path) -> anyhow::Result<Vec<String>> {
    let mut lines_vec: Vec<String> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            lines_vec.push(line);
        }
    }
    Ok(lines_vec)
}

pub fn read_with_callback<F: FnMut(String) -> anyhow::Result<()>>(
    path: &Path,
    map: &mut F,
) -> anyhow::Result<()> {
    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            map(line)?;
        }
    }
    Ok(())
}
