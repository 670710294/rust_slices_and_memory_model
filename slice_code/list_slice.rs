fn main() {
    let v = vec![0, 1, 2, 3, 4, 5, 6];

    let s1 = &v[1..3]; //เริ่มจาก index ที่ 1 -> 3 (1, 2)
    let s2 = &v[..3]; //เริ่มตั้งแต่ index ตัวแรก -> index ที่ 3
    let s3 = &v[1..]; //เริ่มจาก index ที่ 1 -> index ตัวสุดท้าย
    let s4 = &v[..]; //เริ้่มจาก index ตัวแรก -> index ตัวสุดท้าย

    println!("{:?}", s1);
    println!("{:?}", s2);
    println!("{:?}", s3);
    println!("{:?}", s4);
}
