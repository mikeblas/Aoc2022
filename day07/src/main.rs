use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct DirInfo {
    id: i32,
    parent_id: i32,
    name: String,
    children: Vec<i32>,
    size: i32,
    recursive_size: i32,
}

struct DirSpace {
    space: HashMap<i32, DirInfo>,
    rootid: i32,
    nextid: i32,
}

impl DirInfo {

    pub fn new (name: String, parent_id: i32, id: i32) ->Self {

        let ch :Vec<i32> = Vec::new();

        Self {
            id: id,
            parent_id: parent_id,
            name: name,
            children: ch,
            size: 0,
            recursive_size: 0
        }
    }

    pub fn push_child(&mut self, childid: i32) {
        self.children.push(childid);
    }

}

impl DirSpace {
    pub fn new() -> DirSpace {
        let mut h:  HashMap<i32, DirInfo> = HashMap::new();

        let root = DirInfo::new(String::from("/"), -1, 0);
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
            println!("looking at {info} {} for {}", ci.name, name);
            if ci.name == name {
                if ci.size != 0 {
                    panic!();
                }
                return Some(*c);
            }
        }

        None
    }

    pub fn root_size(&self) -> i32 {

        let di = self.space.get(&self.rootid).unwrap();
        di.recursive_size + di.size
    }

    pub fn add_child( &mut self, parent: i32, name: String) -> i32 {

        let di = self.space.get_mut(&parent).unwrap();

        let info: DirInfo = DirInfo::new(name, parent, self.nextid);
        self.nextid += 1;

        di.push_child(info.id);
        // di.children.push(info.id);
        self.space.insert(info.id, info);

        self.nextid-1
    }

    pub fn get_parent(&self, info: i32) -> Option<i32> {

        let di = self.space.get(&info).unwrap();
        Some(di.parent_id)
    }


    pub fn add_size(&mut self, info: i32, size: i32)  {

        let di = self.space.get_mut(&info).unwrap();
        di.size += size;
    }

    pub fn get_recursive_sizes(&mut self) {

        let mut leafs = Vec::new();

        println!("===== find leafs");
        for x in &self.space {
            if x.1.children.len() == 0 {
                leafs.push(x.1.id);
                println!("{},  {}, {}, {}, {}", x.1.id, x.1.name, x.1.recursive_size, x.1.size, x.1.children.len());
            }
        }

        println!("===== work leafs");
        for leaf in leafs {

            let mut total = 0;
            let mut walker = leaf;

            while walker != -1 {
                let di = self.space.get_mut(&walker).unwrap();

                di.recursive_size += total;
                total += di.size;

                println!("{leaf}: {} set to {}", di.name, di.recursive_size);

                walker = di.parent_id;
            }
        }

        println!("===============");
        let mut grand_total = 0;
        for x in &self.space {
            println!("{}, {}, {}", x.1.name, x.1.recursive_size + x.1.size, x.1.children.len());
            if x.1.recursive_size <= 100000 {
                grand_total += x.1.recursive_size;
            }
        }

        println!("grand total {}", grand_total);
    }

    pub fn find_space(&self) {
        let root = self.root_size();
        let free = 70000000 - root;
        let needed = 30000000 - free;
        println!("root is {root}, leaving {free}; need {needed}");

        let mut min_difference = i32::MAX;
        let mut target_size = 0;
        for x in &self.space {
            if x.1.recursive_size + x.1.size > needed {
                if x.1.recursive_size + x.1.size - needed < min_difference {
                    min_difference = x.1.recursive_size + x.1.size - needed;
                    target_size = x.1.recursive_size + x.1.size;
                }
            }
        }

        println!("{} target size", target_size);
    }
}


fn part1() {

    let file = File::open("src/input1.txt")
        .expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let mut dirspace = DirSpace::new();
    let mut current = dirspace.rootid;

    let mut mode = 0;

    let mut flat_size = 0;

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
                if new_name == ".." {
                    current = dirspace.get_parent(current).unwrap();
                } else if new_name == "/" {
                    current = dirspace.rootid;
                } else {
                    println!("want to find {new_name} in {current}");
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
            // println!("mode: {mode}");
            if line.starts_with("dir ") {
                let param_start = line.find("dir ").unwrap();
                let new_name = line[param_start+4..].to_string();
                println!("added {new_name} to {current}");
                dirspace.add_child(current, new_name);
            } else {
                let space = line.find(' ').unwrap();
                let size = line[..space].parse::<i32>().unwrap();
                dirspace.add_size(current, size);
                println!("stored {size} to {current}");
                flat_size += size;
            }
        }
    }

    println!("flat size is {flat_size}");
    dirspace.get_recursive_sizes();

    // part2

    dirspace.find_space();
}


fn main() {
    part1();

    // 151937331 too high
}