// Aldaron's Device Interface - Storage
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// lib.rs

//! Aldaron's Device Interface - Storage (adi_storage) is a Rust library for
//! interfacing with a persistent storage device (ie: hard drive, solid state
//! drive, sd card, flash drive, etc.).

#![doc(
	html_logo_url =
		"https://rawgit.com/aldarons-tech/adi_storage/master/res/icon.png",
	html_favicon_url =
		"https://rawgit.com/aldarons-tech/adi_storage/master/res/symbol.png",
	html_root_url = "http://at.plopgrizzly.tech/utem/"
)]

extern crate whoami;

use std::fs;
use std::path::Path;
use std::io::{ Read, Write };

/// Whether a path is a file or a folder.
pub enum PathType {
	Folder,
	File,
}

/// A persistent storage device.
pub struct Storage {
	// "/app_name.developer/" on Aldaron's OS
	// "/usr/local/share/at_root/app_name.developer/" on Linux
	// "c:/program files/at_root/app_name.developer/" on Windows
	app_root: String,
	// The user currently logged in
	user: String,
}

impl Storage {
	/// Use the `storage!()` macro instead.
	pub fn new(app_name: &str, developer: &str) -> Storage {
		let mut app_root = if cfg!(target_os = "linux") {
			"/usr/local/share/at_root/"
		} else if cfg!(target_os = "windows") {
			"C:/Program Files/at_root/"
		} else if cfg!(target_os = "aldarons_os") {
			"/"
		} else {
			panic!("AA!")
		}.to_string();

		// "/app_name.developer/"
		app_root.push_str(&app_name.to_lowercase());
		app_root.push_str(".");
		app_root.push_str(&developer);
		app_root.push_str("/");

		// "/app_name.developer/user.main/"
		// TODO: Only works/compiles for UNIX currently.
		let mut user = String::new();

		user.push_str(&whoami::username().to_lowercase());
		user.push_str(".");
		user.push_str(&whoami::computer());

		Storage {
			app_root: app_root,
			user: user,
		}
	}

	pub fn sync(&self) {
	}
}

impl ::std::fmt::Display for Storage {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		write!(f, "{{ app_root: \"{}\", user: \"{}\" }}", self.app_root,
			self.user)
	}
}

/// Storage
#[macro_export] macro_rules! storage {
	() => ( {
		let app_name = env!("CARGO_PKG_NAME");
		let developer = include!(concat!(env!("CARGO_MANIFEST_DIR"),
			"/target/res/src/developer.rs"));

		adi_storage::Storage::new(app_name, developer)
	} )
}

/// Save a file.
#[macro_export] macro_rules! file_save {
	(file_name) => ( {
		let app_name = include!(concat!(env!("CARGO_MANIFEST_DIR"),
			"/target/res/src/name.rs"));
		let developer = include!(concat!(env!("CARGO_MANIFEST_DIR"),
			"/target/res/src/developer.rs"));
		let mut app_root = STORAGE_MAIN.to_string();

		// "/app_name.developer/"
		app_root.push_str(app_name);
		app_root.push_str(".");
		app_root.push_str(developer);
		app_root.push_str("/");

		// "/app_name.developer/user.main/"
		// TODO: Only works/compiles for UNIX currently.
		let user = users::get_user_by_uid(users::get_current_uid());
		app_root.push_str(user.unwrap());
		app_root.push_str(".");
		app_root.push_str(hostname::get_hostname().unwrap());
	} )
}

/// Save a file.
pub fn save<P: AsRef<Path>, B: AsRef<[u8]>>(filename: P, data: B) -> () {
	let path = filename.as_ref();
	let parent = path.parent().unwrap();

	if parent.exists() == false {
		mkdir(parent);
	}

	fs::File::create(path).unwrap().write_all(data.as_ref()).unwrap();
}

/// Load a file.
pub fn load<P: AsRef<Path>>(filename: P) -> Vec<u8> {
	let mut file = fs::File::open(filename).unwrap();
	let mut contents = Vec::new();

	file.read_to_end(&mut contents).unwrap();

	contents
}

/// Delete a file.
pub fn rm<P: AsRef<Path>>(path: P) {
	fs::remove_file(path).unwrap();
}

/// Delete a folder and all of it's contents ( use carefully ).
pub fn rmdir<P: AsRef<Path>>(path: P) {
	fs::remove_dir_all(path).unwrap();
}

/// Make a folder.
pub fn mkdir<P: AsRef<Path>>(path: P) {
	fs::create_dir_all(path).unwrap();
}

// Because: https://doc.rust-lang.org/std/fs/fn.rename.html Platform-Specifc...
#[cfg(target_os = "linux")]
fn mv_ll<P: AsRef<Path>>(old: P, new: P) {
	rmdir(&new);
	mkdir(&new);
	fs::rename(old, new).unwrap();
}

/// Move or rename a file ( change it's path ).
pub fn mv<P: AsRef<Path>>(old: P, new: P) {
	mv_ll(old, new);
}

/// Get the permissions of a file.
pub fn get_permissions<P: AsRef<Path>>(name: P) -> fs::Permissions {
	let file = fs::File::open(name).unwrap();
	
	file.metadata().unwrap().permissions()
}

/// Set the permissions of a file.
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

/// Duplicate a file.
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

/// Returns true only if `filepath` exists.
pub fn get_exists(filepath: &str) -> bool {
	Path::new(filepath).exists()
}

/// Get the type of file at `path`, or `None` if there is no file at `path`.
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
