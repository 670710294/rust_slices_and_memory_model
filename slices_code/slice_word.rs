//สร้างกล่องเก็บข้อความ
struct TextAnalyzer{
    text : String
}

//สร้างกล่องใหม่ construct
impl TextAnalyzer{
    // fn new(text: &str) -> TextAnalyzer{
    //     TextAnalyzer{
    //         text : text.to_string(),
    //     }
    // }

    //ใช้คำว่า self จะโยงไปที่ชื่อของ struct เองเลย ถ้ามีการเปลี่ยนชื่อก็แก้เพียงตำแหน่ง struct & impl
    fn new(text: &str) -> Self{ //รับข้อความมาดู(แบบขอยืม)
        Self{
            text : text.to_string(), //เอาข้อความที่ยืมมา คัดลอกเป็นของตัวเอง เพราะยืมนานๆไม่ได้
        }
    }

    //หาคำที่ยาวที่สุด
    fn analyze_word_length(&self, word_slice: &Vec<&str>) -> (usize, String) {
        let mut max_length = 0;
        let mut longest_word = "";

        for &word in word_slice { //หยิบมาดูทีละคำ จากถุงคำที่ส่งเข้ามา
            if word.len() > max_length { //ถ้าคำนี้ยาวกว่าคำก่อนหน้าที่เคยยาวที่สุด
                max_length = word.len(); //อัปปเดตค่า
                longest_word = word; //return
            }
        }

        (max_length, longest_word.to_string())
    }

    //หยิบคำในช่วงที่ต้องการ
    fn get_word_range(&self, start: usize, end: usize) -> Vec<&str>{
        self.text
            .split_whitespace() //เอาคำมาหั่นตรงช่องว่าง แยกเป็นคำๆ
            .collect::<Vec<&str>>() //เอาคำที่หั่นแล้วมาเรียงเป็น list
        [start..end] //หยิบคำตั้งแต่ตำแหน่ง start ถึง end
        .to_vec() //คัดลอกคำพวกนี้มาเป็น list ของตัวเอง
    }
}

fn main(){
    let text = "The quick brown fox jumping over the lazy dog";
    let ta = TextAnalyzer::new(text);

    let word_slice = ta.get_word_range(1, 4);
    println!("Selected words: {:?}", word_slice);

    let (length, word) = ta.analyze_word_length(&word_slice);
    println!("Longest word: {:?} ({:?})", word, length);
}
