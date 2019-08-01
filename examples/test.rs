extern crate easy_awss3;
use easy_awss3::awss3::s3::Awss3Client;
use easy_awss3::client_opt::ClientOpt;

use std::str;

fn main() {

    let tec = Awss3Client::new(
        "whatever your access_key_id",
        "whatever your secret_access_key",
        "whatever your token_id",
        "http://s3.bj.bcebos.com",
        "here is the proxy",
        "here is the user_agent",
        "V2/V4",
        true,
    );

    let sdf = tec.read_object_to_mem("ssssss", "123", 0, 40).unwrap();
    println!("\n\n{:#?}\n\n", str::from_utf8(&sdf).unwrap());

    tec.write_object_with_mem("test-1259750376", "1234", "datas".as_bytes()).unwrap();
    let listobjects = tec.list_objects("test-1259750376").unwrap();
    println!("{:?}", listobjects);
    let del = tec.delete_object("test-1259750376", "1234").unwrap();
    println!("{:?}", del);
    
    let ret_str = tec.query_object_info("test-1259750376", "123").unwrap();
    println!("{:?}", ret_str);

    let mut dwc: f32 = 0.0;
    let dex = tec.read_object_to_file("test-1259750376", "1234", "a.txt", &mut dwc).unwrap();
    println!("{:?}", dex);

    tec.write_object_with_file("test-1259750376", "1234", "a.txt", &mut dwc).unwrap();

    println!("xxxxxxxxsdf");
}
