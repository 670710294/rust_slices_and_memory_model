fn main(){
    let message = String::from("Hello, World!"); //Heap-allocate string
    let m = "Hello, World!"; //ใช้กับ literral String (non heap-allocated string)ได้เหมือนกัน แต่ mut ไม่ได้
    
    let hello = &message[..5];
    let world = &message[7..];

    println!("{:?}", hello);
    println!("{:?}", world);
}

fn main(){
    let mut colors = vec!["red", "blue", "green"]; //mut = สามารถเปลี่ยนค่าในตัวแปรได้หลังจากประกาศแล้ว
    let slice = &mut colors[1..]; //ยืมค่าจา่ก mutated array colors
    slice[0] = "purple"; //เปลี่ยน String ใน index ที่ 0 จาก "blue" เป็น "purple"

    println!("{:?}", slice);
    println!("{:?}", colors);
}
