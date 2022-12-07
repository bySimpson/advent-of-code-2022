use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone, PartialEq)]
pub struct CliFile {
    pub name: String,
    pub size: i32,
}

impl CliFile {
    pub fn new(name: String, size: i32) -> Self {
        Self { name, size }
    }
}

impl PartialEq<String> for CliFile {
    fn eq(&self, other: &String) -> bool {
        self.name == *other
    }
}

impl PartialEq<str> for CliFile {
    fn eq(&self, other: &str) -> bool {
        self.name == *other
    }
}

#[derive(Debug)]
pub struct Folder {
    pub files: Vec<CliFile>,
    pub folders: Vec<Rc<RefCell<Folder>>>,
    pub parent: Weak<RefCell<Folder>>,
    pub name: String,
    total_size: i32,
}

impl Folder {
    pub fn new(name: String) -> Self {
        Self {
            files: vec![],
            folders: vec![],
            parent: Weak::new(),
            name,
            total_size: 0,
        }
    }

    pub fn add_file(&mut self, name: String, size: i32) -> bool {
        self.files.push(CliFile::new(name, size));
        true
    }

    pub fn calculate_size(&mut self) -> i32 {
        let mut score = 0;
        for c_child in self.folders.iter() {
            /*for c_folder in c_child.borrow().files.iter() {
                score += c_folder.size
            }*/
            c_child.borrow_mut().calculate_size();
            score += c_child.borrow().total_size;
        }
        for c_folder in self.files.iter() {
            score += c_folder.size
        }
        self.total_size = score;
        score
    }

    pub fn get_every_size(&self) -> Vec<i32> {
        let mut out_vec: Vec<i32> = vec![];
        for c_child in self.folders.iter() {
            out_vec.append(&mut c_child.borrow().get_every_size());
            out_vec.push(c_child.borrow().total_size);
        }

        out_vec
    }

    pub fn get_folders_smaller_then(&self, max_size: i32) -> Vec<i32> {
        let output = self
            .get_every_size()
            .into_iter()
            .filter(|x| x < &max_size)
            .collect::<Vec<i32>>();
        output
    }

    pub fn get_smallest_directory(&self, total_amount: i32) -> i32 {
        let amount_to_free = (total_amount - self.total_size - 30000000) * -1;
        let mut larger_than_required = self
            .get_every_size()
            .into_iter()
            .filter(|x| x.to_owned() > amount_to_free)
            .collect::<Vec<i32>>();
        larger_than_required.sort();
        larger_than_required[0]
    }
}
