// Online Rust compiler (editor)
// Write and run Rust online using this editor.

fn main() {
    let data = [1,2,3];
    let moved = &data;
    //moved = [1,2,3,4];
    println!("{:?}",data);
    println!("Try clicking the Run button.");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("{r3}");
    
    
    let mut x = 2;
    let y = &mut x;
    *y = 4;
    println!("{y}"); // ถ้าปริ้นบรรทัดนี้จะ error เพราะ y เป็น reference ต้อง dereference ก่อน
    println!("{:?}",x);
    
    
    
}