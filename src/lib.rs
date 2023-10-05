use std::thread;
use std::time::{Instant};
use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jlong;

#[no_mangle]
pub fn Java_com_softwaresmyth_Main_ackermann(_env: JNIEnv, _obj: JObject, m: jlong, n: jlong) -> jlong {
    let result = thread::Builder::new()
        .stack_size(3 * 512 * 1024)
        .spawn(move || ackermann(m, n));
    match result {
        Ok(val) => { val.join().expect("thread returned an error") }
        Err(_) => { -1 }
    }
}

fn ackermann(m: i64, n: i64) -> i64 {
    if 0 == m { return n + 1;}
    if 0 == n { return ackermann(m - 1, 1); }
    ackermann(m - 1, ackermann(m, n - 1))
}

pub fn time_ackermann(m: i64, n: i64) -> i64 {
    let before = Instant::now();
    let result = ackermann(m, n);
    let duration = Instant::now().duration_since(before);

    println!("{:?}", duration);
    result
}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;
    use rand::Rng;

    #[test]
    fn test_m_zero() {
        let mut rng = rand::thread_rng();

        let n1: i64 = rng.gen();
        let n2: i64 = rng.gen();
        let n3: i64 = rng.gen();

        assert_eq!(n1 + 1, ackermann(0, n1));
        assert_eq!(n2 + 1, ackermann(0, n2));
        assert_eq!(n3 + 1, ackermann(0, n3));
    }

    #[test]
    fn test_4_0() {
        // default stack size is 2 MiB
        // need a bigger stack to accommodate this call
        let result = thread::Builder::new()
            .stack_size(1 * 1024 * 1024)
            // .stack_size(8 * 1024 * 1024)
            .spawn(|| ackermann(4, 1));
        match result {
            Ok(val) => { assert_eq!(65533, val.join().expect("thread returned an error")) }
            Err(err) => { println!("{}", err); }
        }
    }
}
