use md5;

fn main() {
    let key = String::from("yzbqklnj");
    let mut index = 0;
    let mut no_match_found = true;
    while no_match_found {
        let secret_key = format!("{}{}", key, index);
        let ref_hash = md5::compute(secret_key.as_bytes());
        let format_hash = format!("{x:x}", x = ref_hash);

        if &format_hash[0..5] == "00000" {
            no_match_found = false;
            println!("{:?}", index)
        } else {
            index += 1;
        }
    }
}
