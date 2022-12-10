use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct DirInfo {
    id: i32,
    parent_id: i32,
    name: String,
    children: Vec<i32>,
}

struct DirSpace<'a> {
    space: HashMap<i32, &'a DirInfo>,
    rootid: i32,
    nextid: i32
}

impl DirInfo {

    pub fn new (name: String, parent_id: i32, id: i32) ->Self {

        let ch :Vec<i32> = Vec::new();

        Self {
            id: id,
            parent_id: parent_id,
            name: name,
            children: ch
        }
    }

    pub fn push_child(&mut self, childid: i32) {
        self.children.push(childid);
    }
}

impl<'a> DirSpace<'a> {
    pub fn new() -> DirSpace<'a> {
        let mut h:  HashMap<i32, DirInfo> = HashMap::new();

        let mut root = DirInfo::new(String::from("/"), -1, 0);
        h.insert(0, root);

        Self {
            space: h,
            nextid: 1,
            rootid: 0
        }
    }

    pub fn find_child(&self, info: i32, name: String) -> Option<i32> {

        let di = self.space.get(&info).unwrap();

        for c in &di.children {
            let ci = self.space.get(c).unwrap();
            if ci.name == name {
                return Some(*c);
            }
        }

        None
    }

    pub fn add_child( &mut self, info: i32, name: String) {

        let mut di = **self.space.get(&info).unwrap();

        let info: DirInfo = DirInfo::new(String::from("/"), info, self.nextid);
        self.nextid += 1;

        di.push_child(info.id);
        // di.children.push(info.id);
        self.space.insert(info.id, &info);
    }
}


fn part1() {

    let file = File::open("src/sample.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);


    let mut dirspace = DirSpace::new();
    let mut current = dirspace.rootid;

    let mut mode = 0;

    for line in reader.lines() {

        let line = line.unwrap();

        if line.starts_with("$ ") {
            mode = 0;
        }

        if mode == 0 {
            if line.starts_with("$ cd") {
                let param_start = line.rfind(" ").unwrap();
                let new_name = &line[param_start + 1..];
                println!("Directory: '{new_name}'");
                if new_name == "/" {
                    current = dirspace.rootid;
                } else {
                    current = dirspace.find_child(current, new_name.to_string()).unwrap();
                }
            } else if line.starts_with("$ ls") {
                println!("list: {line}");
                mode = 1;
            } else {
                println!("unknown: {line}");
            }
        }
        else if mode == 1 {
            println!("mode: {mode}");
            if line.starts_with("dir ") {
                let param_start = line.find(" ").unwrap();
                dirspace.add_child(current, line[param_start+1..].to_string());
            }
        }
    }
}

fn part2() {

}

fn main() {
    part1();
    part2();
}