extern crate easy_awss3;
use easy_awss3::awss3_client::Awss3Client;
use easy_awss3::awss3_opt::Awss3Opt;

use std::str;

fn main() {

    let tec = Awss3Client::new(
        "whatever your access_key_id",
        "whatever your secret_access_key",
        "whatever your token_id",
        "http://s3.bj.bcebos.com",
        "here is the proxy",
        "here is the user_agent",
        false,
    );

    let sdf = tec.read_object_to_mem("bucket", "object", 0, 40).unwrap();
    println!("\n\n{:#?}\n\n", str::from_utf8(&sdf).unwrap());

    tec.write_object_with_mem("bucket", "object", "datas".as_bytes()).unwrap();
    let listobjects = tec.list_objects("bucket").unwrap();
    println!("{:?}", listobjects);
    tec.delete_object("bucket", "object").unwrap();

    let ret_str = tec.query_object_info("bucket", "object").unwrap();

    let mut dwc: f32 = 0.0;
    tec.write_object_with_file("bucket", "object", "file path", &mut dwc).unwrap();
    println!("{:?}", ret_str);

    let dex = tec.read_object_to_file("bucket", "object", "local file path", &mut dwc).unwrap();
    println!("{:?}", dex);

    println!("xxxxxxxxsdf");
}
