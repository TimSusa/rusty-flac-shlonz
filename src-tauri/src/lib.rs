
pub mod models;
pub mod schema;

use crate::models::{NewPost, Post};
use claxon::FlacReader;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
extern crate walkdir;
use walkdir::WalkDir;

use std::path::Path;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}




pub fn get_all_db_items() -> Vec<Post> {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    //println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("-----------\n");
    //     println!("{}", post.body);
    // }
    return results;

        // The following will return all users as a `QueryResult<Vec<User>>`
}

// pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str)  {
//     use crate::schema::posts;

//     let new_post = NewPost { title, body };


//     diesel::insert_into(posts::table)
//     .values(&new_post)
//     .get_result(conn)
//     .expect("Error saving new post");

//     // diesel::insert_into(posts::table)
//     //     .values(&new_post)
//     //     .execute(conn)
//     //     .expect("Error saving new post");

//     //posts::table.order(posts::id.desc()).first(conn).unwrap()

// }

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str)  {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn).expect("error");
//         .returning(Post::as_returning())
//         .get_result(conn)
//         .expect("Error saving new post")
}


pub fn write_info_to_db(file_path: &str, db: & mut SqliteConnection) -> Result<(), String> {
    let reader = FlacReader::open(file_path).map_err(|e| e.to_string())?;
    let md5: [u8; 16] = reader.streaminfo().md5sum;
    let md5_string = md5.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>().join("");
    if md5.len() > 0 {
        dbg!("md5 {}", md5_string.clone());
        create_post(db, &file_path, &md5_string);
        println!("\nSaved draft {}", file_path);
    } 
//    // Print all other tags
//     for (name, value) in reader.tags() {
//         println!("{}: {}", name, value);
//     }

    Ok(())
}

pub fn find_flac_files(dir_path: &str) -> Vec<String> {
    let mut flac_files = Vec::new();

    // Recursively walk through the directory
    for entry in WalkDir::new(dir_path).into_iter().filter_map(Result::ok) {
        // Check if the entry is a file and has a .flac extension
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "flac" {
                    flac_files.push(entry.path().display().to_string());
                }
            }
        }
    }

    flac_files
}



pub fn scan_by_path_write_db(dir_path: &String) {
	// Check if the provided path is a directory
	if !Path::new(dir_path).is_dir() {
		eprintln!("Error: {} is not a directory", dir_path);
		std::process::exit(1);
	}

	// Collect all FLAC files in the directory and its subdirectories
	let flac_files = find_flac_files(dir_path);

	let connection = &mut establish_connection();

	// Print the metadata of FLAC files
	if flac_files.is_empty() {
		println!("No FLAC files found in the directory.");
	} else {
		println!("FLAC files found in the directory:");
		for file in flac_files {
			println!("Processing file: {}", file);

			match write_info_to_db(&file, connection) {
				Ok(_) => (),
				Err(e) => eprintln!("Error reading metadata from {}: {}", file, e),
			}
		}
	}
}
