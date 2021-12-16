#[cfg(test)]
#[test]
fn g_uuid() {
    // use std::{thread, time::Duration};

    for i in 0..100 {
        let guid = scru128::scru128_string();
        println!("{},{},{}", i, guid, guid.len());
    }
    // thread::sleep(Duration::from_secs(1000));
}

#[test]
fn rand_string() {
    // use std::{thread, time::Duration};

    for i in 0..10 {
        let r_s = rand::random::<char>();
        println!("{},{}", i, r_s);
    }
    // thread::sleep(Duration::from_secs(1000));
}

#[test]
fn t_md5() {
    use md5;
    use std::fmt::Write;
    let tt = md5::compute("1e321");
    println!("{:?},{}", tt, tt.len());
    let ttt = tt.to_vec();
    let mut signature_string = String::new();
    for a in ttt.iter() {
        //println!(" N: {:x?}", a);

        //signature_string.push(a);

        write!(signature_string, "{:02x}", a).unwrap();
    }
    // let ss = String::from_utf8_lossy(&tt);

    println!("tt{:?}", signature_string);
}

#[test]
fn main_ax() {
    use std::fmt::Write;

    let mut signature_string = String::new();

    let signature_code = [
        177, 187, 102, 36, 165, 137, 39, 63, 52, 197, 173, 13, 168, 216, 95, 3, 175, 113, 213, 98,
        52, 77, 175, 152, 79, 188, 119, 141, 52, 19, 19, 53,
    ];

    //for a in signature_code().iter() {

    for a in signature_code.iter() {
        //println!(" N: {:x?}", a);

        //signature_string.push(a);

        write!(signature_string, "{:02x}", a);
    }

    println!(
        "the entire array HEX as a single string: {}",
        signature_string
    );
}
