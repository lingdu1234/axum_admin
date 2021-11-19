#[cfg(test)]
#[test]
fn g_uuid() {
    // use std::{thread, time::Duration};

    for i in 0..100 {
        let guid = scru128::scru128();
        println!("{},{},{}", i, guid, guid.len());
    }
    // thread::sleep(Duration::from_secs(1000));
}
