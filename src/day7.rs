use std::collections::HashMap;

#[derive(Debug)]
enum Elem {
    Dir(String),
    File((String, usize)),
}

fn get_size(dir: &str, fs: &HashMap<String, Vec<Elem>>) -> usize {
    let mut size = 0;
    for e in fs.get(dir).unwrap() {
        size += match e {
            Elem::Dir(d) => get_size(d, &fs),
            Elem::File(f) => f.1,
        }
    }
    size
}

fn parse(s: &String) -> HashMap<String, Vec<Elem>> {
    let mut tree = HashMap::new();
    let mut lines = s.lines();

    let mut current_folders: Vec<String> = vec![];
    let mut current_folder = String::new();
    loop {
        let l = match lines.next() {
            Some(l) => l,
            None => break,
        };
        if l.starts_with("$ cd ..") {
            current_folders.pop();
            current_folder.clear();
            for f in &current_folders {
                current_folder += f;
                current_folder += "/";
            }
            continue;
        }
        else if l.starts_with("$ cd ") {
            current_folders.push(String::from(l.strip_prefix("$ cd ").unwrap()));
            current_folder.clear();
            for f in &current_folders {
                current_folder += &f;
                current_folder += "/";
            }
            tree.insert(current_folder.clone(), vec![]);
            // for _ in 0..current_folders.len()-1 {print!("\t")}
            // println!(" - {}", &current_folder);
        }
        else if l.starts_with("$ ls") {continue;}
        else if l.starts_with("dir ") { //dirs
            let mut folder = current_folder.clone();
            folder += l.strip_prefix("dir ").unwrap();
            folder +=  "/";
            tree.get_mut(&current_folder).unwrap().push(Elem::Dir(folder));
            // for _ in 0..current_folders.len() {print!("\t")}
            // println!("dir: {}", l.strip_prefix("dir ").unwrap());
        } else {    //files
            let mut n = l.split(" ");
            let size = n.next().unwrap().parse().unwrap();
            let name = String::from(n.next().unwrap());
            // for _ in 0..current_folders.len() {print!("\t")}
            // println!("file: {} | {}", &name, size);
            tree.get_mut(&current_folder).unwrap().push(Elem::File((name, size)));
        }
    }
    tree
}

pub fn chall_1(s : &String) -> usize {
    let slash = parse(s);

    let mut total = 0;
    for (k, _) in &slash {
        let size = get_size(&k, &slash);
        if size <= 100_000 {
            total += size;
        }
    }
    total
}

pub fn chall_2(s : &String) -> usize {
    let slash = parse(s);

    let unused = 70_000_000 - get_size("//", &slash);
    let target_size = 30_000_000 - unused;
    let mut min = 70_000_000;

    for (k, _) in &slash {
        let size = get_size(&k, &slash);
        if size > target_size && size < min {
            min = size;
        }
    }
    min
}