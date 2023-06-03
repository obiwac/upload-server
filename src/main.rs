#[macro_use] extern crate rocket;

use std::fs;

use rocket::{data::Data, http::{ContentType, Header}, fairing::{Kind, Info, Fairing}, Request, Response};
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField};

const OUT_DIR: &str = "out";

#[options("/api/upload")]
fn upload_options() {
}

#[post("/api/upload", data = "<data>")]
async fn upload(content_type: &ContentType, data: Data<'_>) {
	let mut opts = MultipartFormDataOptions::new();

	// this will allow 1000 files maximum
	// AFAIK, there's no way to accept an infinite number of files
	// TODO maybe that's something that can be improved upstream?

	for _ in 0..1000 {
		let field = MultipartFormDataField::file("files")
			.size_limit(u64::MAX);

		opts.allowed_fields.push(field);
	}

	// create output directory if it doesn't already exist

	if fs::metadata(OUT_DIR).is_err() {
		fs::create_dir(OUT_DIR)
			.unwrap_or_else(|e| panic!("can't create {}: {:?}", OUT_DIR, e));
	}

	// parse data into output directory

	opts.temporary_dir = OUT_DIR.into();
	let form_data = MultipartFormData::parse(content_type, data, opts).await.unwrap();

	// go through files and rename them correctly

	for file in &form_data.files["files"] {
		let filename = file.file_name.as_ref().unwrap();
		let path = format!("{}/{}", OUT_DIR, filename);

		fs::rename(&file.path, &path)
			.unwrap_or_else(|e| panic!("can't move {} to {}: {:?}", file.path.to_str().unwrap(), &path, e));
	}
}

// allow CORS

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
	fn info(&self) -> Info {
		Info {
			name: "CORS fairing",
			kind: Kind::Response,
		}
	}

	async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response) {
		res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
		res.set_header(Header::new("Access-Control-Allow-Methods", "POST"));
		res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
		res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
	}
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach(Cors)
		.mount("/", routes![upload_options, upload])
}
