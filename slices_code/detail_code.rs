//การจองแบบ stack vs heap
//Ownership: กฎ 3 ข้อ (แต่ละค่ามีเจ้าของเดียว, ย้ายเจ้าของได้, หมดสโคปแล้วถูก drop)
//Borrowing: &T (immutable) vs &mut T (mutable), กฎ "หนึ่ง mutable หรือหลาย immutable"
//Lifetimes เบื้องต้น — ทำไม Rust ต้องรู้ว่าข้อมูลอยู่ได้นานแค่ไหน
fn main() {
    let s1 = String::from("hello"); // heap-allocated
    let s2 = s1; // move, s1 ใช้ไม่ได้อีก
    // println!("{}", s1); // compile error!
    
    let x = 5; // stack, Copy trait
    let y = x; // copy, x ยังใช้ได้
}

//Fat Pointer vs Thin Pointer
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];           // &[T] แบบ i32
    
    let text = "hello world";
    let str_slice: &str = &text[0..5];         // &str (เป็น &[T] เวอร์ชันพิเศษของ u8)
    
    println!("size of &[i32]: {}", std::mem::size_of::<&[i32]>()); // 16 Bytes เป็น fat pointer(pointer(8)+length(8))
    println!("size of &str: {}",   std::mem::size_of::<&str>());   // 16 Bytes (โครงสร้างเหมือนกัน)
    println!("size of &i32: {}",   std::mem::size_of::<&i32>());   // 8 bytes เป็น thin pointer(pointer(8))    
}


//เปรียบเทียบ &str (string slice) กับ &[T] (generic slice)
fn main() {
    //&[T] : generic slice
    let numbers = [10, 20, 30, 40, 50];
    let num_slice: &[i32] = &numbers[1..4];
    
    println!("num_slice: {:?}", num_slice);      // [20, 30, 40]
    println!("len: {}", num_slice.len());          // 3 (จำนวน element)
    
    //String ใน Rust เก็บข้อมูลภายในเป็น Vec<u8> (bytes ดิบ) ที่เข้ารหัสแบบ UTF-8 ไม่ใช่อาร์เรย์ของตัวอักษร
    //ดังนั้นเวลาเขียน &s[0..1] มันไม่ได้หมายถึง "ตัวอักษรตัวที่ 0 ถึง 1" แต่หมายถึง "byte ที่ 0 ถึง 1"
    //&str : string slice (สไลซ์เฉพาะของ str)
    let text = String::from("Hello Rust");
    let str_slice: &str = &text[0..5];
    
    println!("str_slice: {}", str_slice);           // Hello
    println!("len: {}", str_slice.len());            // 5 (จำนวน byte)

    //ถ้าต้องการ slice String แบบ generic slice ล่ะ
    let s = String::from("hello");

    let byte_slice: &[u8] = s.as_bytes();       // แปลงทั้ง String เป็น &[u8]
    println!("{:?}", byte_slice);                 // [104, 101, 108, 108, 111]

    let partial: &[u8] = &s.as_bytes()[0..2];    // slice บางส่วน
    println!("{:?}", partial);                     // [104, 101]  ('h', 'e')
}

//String (owned, heap) vs &str (borrowed view)
//String literal "hello" คือ &'static str — อยู่ใน binary ไม่ใช่ heap
fn main() {
    let s = String::from("สวัสดี world");
    let hello = &s[0..0]; // panic! เพราะ "ส" ใช้ 3 bytes, index 1 ตัดกลางตัวอักษร
    //ภาษาไทยเป็น multi-byte ใช้ 3 bytes ในการเข้ารหัส UTF-8 (ภาษาอังกฤษใช้แค่ 1)
    
    // let bad = &s[0..1]; // panic! เพราะตัดกลาง multi-byte char
    
    for (i, c) in s.char_indices() {
        println!("{}: {}", i, c);
    }
}

//Slice กับ Borrow Checker
fn main() {
    let mut v = vec![1, 2, 3, 4, 5]; 
    let slice = &v[..2]; //ยืมแบบ immutable
    v.push(6); // //ยืมแบบ immutable: compile error! slice ยัง borrow อยู่
    println!("{:?}", slice);
}

//Slicing Patterns และ Methods ที่ใช้บ่อย
//Split at แบ่งครึ่งตำแหน้งที่ index
fn main (){
    let data = [1, 2, 3, 4, 5, 6, 7];
    let (left, right) = data.split_at(3);
    // left  = [1, 2, 3]
    // right = [4, 5, 6, 7]
    let Some((first, rest)) = data.split_first() else { return }; // 1, [2,3,4,5,6,7]
    let Some((last, rest)) = data.split_last() else { return };   // 7, [1,2,3,4,5,6]
}

//chunk แบ่งเป็นก้อนเท่าๆกัน
fn main(){

    let data = [1, 2, 3, 4, 5, 6, 7];
    for chunk in data.chunks(3) {
        println!("{:?}", chunk);
    }
    // [1, 2, 3]
    // [4, 5, 6]
    // [7]          <- เหลือไม่ครบก็เอาเท่าที่มี
    
    let mut chunks_iter = data.chunks_exact(3); //ตัดก้อนที่ไม่ครบออก
    for chunk in &mut chunks_iter {
        println!("{:?}", chunk);
    }

    println!("เหลือไม่ครบ: {:?}", chunks_iter.remainder()); // [7]

    //as_chunks
    let (chunks, remainder) = data.as_chunks::<3>();
    println!("{:?}", chunks);    // [[1, 2, 3], [4, 5, 6]]
    println!("{:?}", remainder); // [7]
}

//windows เลื่อนหน้าต่างทีละ 1
fn main(){
    let data = [1, 2, 3, 4, 5];
    for w in data.windows(2) {
        println!("{:?}", w);
    }
    // [1, 2]
    // [2, 3]
    // [3, 4]
    // [4, 5]

    //array_windows
    let mut iter = data.array_windows::<2>();
    println!("{:?}", iter.next()); // Some(&[1, 2])
    println!("{:?}", iter.next()); // Some(&[2, 3])
}

//Pattern Matching ดูค่าหัวท้าย(ไม่สนใจค่ากลาง)
fn main() {
    let data = [1, 2, 3, 4, 5, 6, 7];
    match &data {
    [first, .., last] => println!("first={}, last={}", first, last),
    _ => {}
    }
    // first=1, last=7
}

//iter() vs iter_mut()
fn main(){
    let data = &[1, 2, 4];
    // iter() = แค่ดู ห้ามแก้
    for n in data.iter() {
        println!("{}", n);
    }

    // iter_mut() = ดูแล้วแก้ค่าได้เลย
    let data = &mut [1, 2, 4];
    for n in nums.iter_mut() {
        *n += 2; // ต้องมี * ข้างหน้าเวลาจะแก้ค่า
    }
    // nums กลายเป็น [3, 4, 6]
}
