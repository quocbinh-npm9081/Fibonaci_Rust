use std::io;

fn main() {
    loop {
        println!("FIBONACI");
        println!("Nhap so cua ban( nhap -9 de thoat):");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: i32 = match n.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        if n == -9 {
            break;
        } else {
            let mut count: i32 = 0;

            while count <= n {
                //n=2    0 1 1
                let number = fibonaci(count);
                println!("-{}", number);
                count += 1;
            }
        }
    }
}

fn fibonaci(_i: i32) -> i32 {
    //f0=0 f1=1 f2=1 f3=2
    let mut f0 = 0;
    let mut f1 = 1;
    let mut f_n = 1;
    if _i < 0 {
        return -1;
    } else if _i == 0 || _i == 1 {
        return _i;
    } else {
        let mut count: i32 = 2;
        while count < _i {
            f0 = f1;
            f1 = f_n;
            f_n = f0 + f1;
            count += 1;
        }
    }
    return f_n;
}

//n=1
//0
//n=2
//1
//n=3
//1 1 2
