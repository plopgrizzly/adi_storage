// lib.rs
// Aldaron's Device Interface / Storage
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

use std::fs;
use std::path::Path;
use std::io::{ Read, Write };

pub enum PathType {
	Folder,
	File,
}

pub fn save<P: AsRef<Path>, B: AsRef<[u8]>>(name: P, data: B) -> () {
	let path = name.as_ref();
	let parent = path.parent().unwrap();

	if parent.exists() == false {
		mkdir(parent);
	}

	fs::File::create(path).unwrap().write_all(data.as_ref()).unwrap();
}

pub fn load<P: AsRef<Path>>(name: P) -> Vec<u8> {
	let mut file = fs::File::open(name).unwrap();
	let mut contents = Vec::new();

	file.read_to_end(&mut contents).unwrap();

	contents
}

pub fn rm<P: AsRef<Path>>(path: P) {
	fs::remove_file(path).unwrap();
}

pub fn rmdir<P: AsRef<Path>>(path: P) {
	fs::remove_dir_all(path).unwrap();
}

pub fn mkdir<P: AsRef<Path>>(path: P) {
	fs::create_dir_all(path).unwrap();
}

// Because: https://doc.rust-lang.org/std/fs/fn.rename.html Platform-Specifc...
#[cfg(target_os = "linux")]
fn mvto_ll<P: AsRef<Path>>(old: P, new: P) {
	rmdir(&new);
	mkdir(&new);
	fs::rename(old, new).unwrap();
}

/// Move or rename a file ( change it's path ).
pub fn mvto<P: AsRef<Path>>(old: P, new: P) {
	mvto_ll(old, new);
}

/// 
pub fn get_permissions<P: AsRef<Path>>(name: P) -> fs::Permissions {
	let file = fs::File::open(name).unwrap();
	
	file.metadata().unwrap().permissions()
}

/// 
pub fn set_permissions<P: AsRef<Path>>(name: P, permissions: fs::Permissions) -> () {
	let file = fs::File::open(name).unwrap();

	file.set_permissions(permissions).unwrap()
}

// Remove first folder in relative path
fn fnrm_first<P: AsRef<Path>>(input: P) -> String {
	let input = input.as_ref().to_str().unwrap();
	let index = input.find('/').unwrap();
	let mut input = input.to_string();
	let t: String = input.drain(index+1..).collect();

	t
}

pub fn copy<P: AsRef<Path>>(src: P, dst: P) -> Result<(), String> {
	let src = src.as_ref();
	let dst = dst.as_ref();

	if let Some(pt) = path_type(src) {
		match pt {
			PathType::File => {
				let permissions = get_permissions(src);
				let data = load(src);

				save(dst, data.as_slice());
				set_permissions(dst, permissions);
				Ok(())
			}
			PathType::Folder => {
				if let Ok(dir_iter) = fs::read_dir(src) {
					for entry in dir_iter {
						if let Ok(entry) = entry {
							let path = entry.path();
							let apnd = fnrm_first(&path);
							let dest = dst.join(&apnd);

							copy(path, dest)?;
						} else {
							return Err("intermitten\
								t io".to_string())
						}
					}
					Ok(())
				} else {
					Err(format!("Couldn't copy folder {:?} \
						because it lacks read \
						permission", src))
				}
			}
		}
	} else {
		Err(format!("Couldn't copy {:?} because it doesn't exist.", src))
	}
}

pub fn get_exists(name: &str) -> bool {
	Path::new(name).exists()
}

pub fn path_type<P: AsRef<Path>>(path: P) -> Option<PathType> {
	let path = path.as_ref();

	if path.exists() == false {
		None
	} else if path.is_file() == true {
		Some(PathType::File)
	} else if path.is_dir() == true {
		Some(PathType::Folder)
	} else {
		panic!("Filesystem contains mysterious entity (Not a file or a \
			folder)!");
	}
}
