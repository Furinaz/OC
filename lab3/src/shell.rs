use core::ptr::null_mut;
use crate::{print, println, clear};
use crate::vga_buf::SCREEN;
use pc_keyboard::DecodedKey;
use lazy_static::lazy_static;


lazy_static! {
    static ref SH: spin::Mutex<Shell> = spin::Mutex::new({
        let mut sh = Shell::new();
        sh
    });
}

pub fn handle_keyboard_interrupt(key: DecodedKey) {
    match key {
        DecodedKey::Unicode(c) => SH.lock().on_key_pressed(c as u8),
        DecodedKey::RawKey(rk) => {}
    }
}

const BUF_WIDTH: u32 = 80;
const COMMANDS: [&str; 6] = ["cur_dir", "make_dir ", "change_dir ", "remove_dir ", "dir_tree", "clear"];
const MAX_FOLDER_NAME_LENGTH: usize = 10;
const MAX_INNER_FOLDERS: usize = 10;

#[derive(Copy, Clone)]
struct Folder {
    pub exist: bool,
    pub parent: i32,
    pub child_indexs: [i32; MAX_INNER_FOLDERS],
    pub name: [u8; MAX_FOLDER_NAME_LENGTH]
}

impl Folder {
    pub fn set_exist(&mut self, value: bool){
        self.exist = value;
    }

    pub fn set_parent(&mut self, value: i32){
        self.parent = value;
    }

    pub fn set_name(&mut self, value: [u8; MAX_FOLDER_NAME_LENGTH]){
        self.name = value;
    }
}

struct Shell {
    buf: [u8; 80],
    buf_len: usize,
    current_dir: usize,
    dirs: [Folder; 20]
}

impl Shell {

    pub fn new() -> Shell {

        Shell {
            buf: [0; 80],
            buf_len: 0,
            current_dir: 0,
            dirs: [Folder{exist: false, parent: -1, child_indexs: [-1; MAX_INNER_FOLDERS], name: [b'e'; MAX_FOLDER_NAME_LENGTH]}; 20]
        }
    }

    pub fn on_key_pressed(&mut self, key: u8) {
        match key {
            b'\n' => {
                self.execute_command();
                self.buf = [0; 80];
                self.buf_len = 0;
            }
            _ => {
                self.buf[self.buf_len] = key;
                self.buf_len += 1;
                print!("{}", key as char);
            }
        }
    }

    fn execute_command(&mut self){
        self.dirs[0].set_exist(true);
        let new_name =[b'r', b'o', b'o', b't', b'\0', b' ', b' ', b' ', b' ', b' '];
        self.dirs[0].set_name(new_name);

        if self.buf_len == 0 { return; }
        let mut input_command: &str = "";

        for command in COMMANDS {
            let mut command_match: bool = true;
            let command_in_arr = command.as_bytes();
            for i in 0..command_in_arr.len() {
                if self.buf[i] == command_in_arr[i] as u8 {}
                else { command_match = false; break; }
            }
            if command_match { input_command = command; break; }
        }

        if input_command == "" {
            print!("\n[err] There is no such command '");
            for i in 0..self.buf_len {
                print!("{}", self.buf[i] as char);
            }
            println!("'");
            return;
        }

        match input_command {
            "cur_dir" => {
                print!("\n/");
                self.print_dir_name(self.current_dir);
                println!();
            },
            "make_dir " => {
                let args = self.get_command_args(input_command);

                for child_index in self.dirs[self.current_dir].child_indexs {
                    if child_index >= 0{
                        if self.dirs[child_index as usize].name == args {
                            println!("\n[err] Directory with same name is already existed");
                            return;
                        }
                    }
                }

                let current_dir_empty_subfolder_indx = self.find_empty_subfolder();
                if current_dir_empty_subfolder_indx >= 0 {
                    let empty_folder_indx = self.find_empty_folder();
                    if empty_folder_indx < 0 { println!("\n[err] You can't create more than 20 folders"); }
                    else {
                        self.dirs[self.current_dir].child_indexs[current_dir_empty_subfolder_indx as usize] = empty_folder_indx;
                        self.dirs[empty_folder_indx as usize] = Folder{
                            exist: true,
                            parent: self.current_dir as i32,
                            child_indexs: [-1; MAX_INNER_FOLDERS],
                            name: args
                        };
                        print!("\n[ok] Created new dir '");
                        self.print_dir_name(empty_folder_indx as usize);
                        println!("'")
                    }
                } else { println!("\n[err] You can't create more than 10 subfolders, try create subfolder in other directory") }
            },
            "change_dir " => {
                let args = self.get_command_args(input_command);
                if args[0] == b'.' {
                    let parent_folder = self.dirs[self.current_dir].parent;
                    if parent_folder >= 0 {
                        self.current_dir = parent_folder as usize;
                        print!("\n[ok] Changed current dir to '");
                        self.print_dir_name(self.current_dir as usize);
                        println!("'");
                        return;
                    }
                    else { println!("[err] There is no parent folder") }
                }

                let child_dir_index = self.find_child_dir_by_name(args);
                if child_dir_index >= 0 {
                    self.current_dir = child_dir_index as usize;
                    print!("\n[ok] Changed current dir to '");
                    self.print_dir_name(child_dir_index as usize);
                    println!("'");
                    return;
                }
                println!("[err] There is no such directory");
            },
            "remove_dir " => {
                let args = self.get_command_args(input_command);

                for i in 0..self.dirs[self.current_dir].child_indexs.len() {
                    let child_index = self.dirs[self.current_dir].child_indexs[i];
                    if child_index >= 0{
                        if self.dirs[child_index as usize].name == args {
                            self.dirs[self.current_dir].child_indexs[i] = -1;
                            for deleted_child in self.dirs[child_index].child_indexs {
                                if deleted_child >= 0 {
                                    self.dirs[deleted_child as usize] = Folder {
                                        exist: false,
                                        parent: -1,
                                        child_indexs: [-1; MAX_INNER_FOLDERS],
                                        name: [b' '; MAX_FOLDER_NAME_LENGTH]
                                    };
                                }
                            }
                            self.dirs[child_index as usize] = Folder {
                                exist: false,
                                parent: -1,
                                child_indexs: [-1; MAX_INNER_FOLDERS],
                                name: [b' '; MAX_FOLDER_NAME_LENGTH]
                            };
                            println!("\n[ok] Successful deleted");
                            return;
                        }
                    }
                }
                println!("[err] There is no such directory");
            },
            "dir_tree" => {
                println!();
                self.print_dir_tree(0, 0);
            },
            "clear" => { clear!(); },
            _ => {}
        }
    }

    fn find_child_dir_by_name(&mut self, name: [u8; MAX_FOLDER_NAME_LENGTH]) -> i32{
        for child_index in self.dirs[self.current_dir].child_indexs {
            if child_index >= 0{
                if self.dirs[child_index as usize].name == name {
                    return child_index;
                }
            }
        }
        return -1;
    }

    fn print_dir_tree(&mut self, start_folder: usize, depth: u16){
        for i in 0..depth { print!(" ") }
        self.print_dir_name(start_folder);
        println!();
        for child_index in (self.dirs[start_folder] as Folder).child_indexs{
            if child_index > 0 {
                if (self.dirs[child_index as usize] as Folder).exist { self.print_dir_tree(child_index as usize, depth + 1) }
            }
        }
    }

    fn find_empty_subfolder(&mut self) -> i8{
        let subfolders = (self.dirs[self.current_dir] as Folder).child_indexs;
        for i in 0..subfolders.len() {
            if subfolders[i] < 0 { return i as i8; }
        }
        return -1;
    }

    fn find_empty_folder(&mut self) -> i32 {
        for i in 0..self.dirs.len() {
            if !(self.dirs[i] as Folder).exist { return i as i32; }
        }
        return -1;
    }

    fn get_command_args(&mut self, command: &str) -> [u8; MAX_FOLDER_NAME_LENGTH]{
        let mut result: [u8; MAX_FOLDER_NAME_LENGTH] = [b' '; MAX_FOLDER_NAME_LENGTH];

        if command == COMMANDS[1] || command == COMMANDS[2] || command == COMMANDS[3] {
            if self.buf_len <= command.len() {
                println!("\n[err] This command require args")
            } else {
                let args_length = self.buf_len - command.len();
                if args_length > MAX_FOLDER_NAME_LENGTH {
                    println!("\n[err] This args too large, max length is 10");
                } else {
                    let mut y: usize = 0;
                    for i in command.len()..self.buf_len {
                        result[y] = self.buf[i];
                        y += 1;
                    }
                    if y < MAX_FOLDER_NAME_LENGTH-1 { result[y] = b'\0'; }
                }


            }
        }
        return result;
    }

    fn print_dir_name(&mut self, dir_index: usize){
        for char in (self.dirs[dir_index] as Folder).name{
            if char == b'\0' { return; }
            print!("{}", char as char);
        }
    }
}
