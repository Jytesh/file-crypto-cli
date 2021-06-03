use std::fs::read_dir;

pub fn readdir(path: String) -> Result<Vec< String>, std::io::Error> {
    let mut array = vec![];
    let entries = read_dir(path)?;
    for path in entries {
        let file = path?.path();
        let file = file.display();
        array.push(format!("{}", file))
    }
    return Ok(array);
}
