# Rust Tutorial Project — Principles of Programming Languages

> **สำหรับนักศึกษา:** ใช้ไฟล์นี้เป็น Template สำหรับจัดทำบทเรียน Rust ของกลุ่ม  
> **Topic No.:** `13`  
> **Topic Name:** `[Slices & Memory Model]`  
> **Group No.:** `13`  

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | `นายปภังกร มงคลนรกิจ` | `670710293` | `@670710293` | Concept + Code |
| 2 | `นางสาวศิริกานต์ หรุ่นมาบแค` | `670710294` | `@670710294` | Code + Demo |
| 3 | `นายสิรวิชญ์ เชี่ยวชาญ` | `670610296` | `@670710296` | Rust vs Other Language + PPL |
| 4 | `นายวีรภัทร พุฒหอม` | `670710336` | `@[username]` | Exercises + Common Mistakes |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`
---

## 3. Introduction

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

`Topic นี้จะพูดถึงการแบ่งย่อยในตัวแปรต่างๆ ไม่ว่าจะเป้น List , Array , String หรืออื่นๆ และการจัดการ Memory ของภาษา Rust`

`เริ่มที่การจัดการ Memory หากไม่มีการจัดการ Memory แบบ Rust`
1) [ตัวแปรข้อมูลจะไม่ปลอดภัย เนื่องจากใครๆก็มาหยิบไปใช้ได้ แก้ข้อมูลได้เสมอ]
2) [หากไม่มี memory Model ตัวข้อมูลจะมั่วซั่วไปหมด เก็บที่ไหนไปเรื่อย และจะทิ้งเป็น Garbage ไว้ใน Ram ซึ่งจะทำให้เปลืองทรัพยากรมากๆ]
3) [Slicing ช่วยให้ลดการจองพื้นที่ Ram แบบไม่จำเป็นทิ้งไป]  

`ส่วนในพาร์ทของ Memory Model ใน Rust จะมี Stack + Heap`  
`โดย Memory Model : ชุดการจัดเก็บข้อมูล/ตัวแปร การใช้งาน การคืนความจำ โดยใช้หลัก Ownership , Borrowing  และ Lifetime`
1) `Stack ใช้ Concept แบบ LIFO`  
`จะเก็บค่าตัวแปรที่รู้ขนาดแน่นอนเท่านั้น เช่น integer,อาเรย์ที่กำหนดขนาดแล้ว`  
2) `Heap ใช้เก็บค่าตัวแปรที่ไม่ทราบขนาดแน่นอน ( เพิ่มขึ้นหรือลดลงได้ตอน Compile )`  
` โดยหลักการมันจะเก็บตัว pointer กับชื่อของตัวแปรไว้ที่ Stack แล้วให้ชี้มาที่ Heap ของค่านั้นๆ `  
` จะเก็บตัวแปรประเภท String , Dynamic Collection เป็นต้น `  


`ส่วนตัวของ Slices คือการแบ่งย่อยจากตัวเซตข้อมูลหลัก ไม่ว่าจะเป็น List , Array หรือ String เป็นต้น`  
`ที่ช่วยทำให้เกิด Zero - Heap Allocation หรือก็คือ ไม่เกิดการจองข้อมูลใน Heap เพิ่มเติม`  
`(แต่ยังคงเก็บใน stack นิดนึงนะ คือชื่อกับ Pointer เพื่อบ่งบอกว่าเราชี้ไปที่ Heap จุดไหน)`  

`เช่น เรามี List หรือ Array ที่มีข้อมูลเป็น [2,3,5,6,7] แต่เราต้องการตั้งแต่ตัวที่ 3 ( คือเลข 5 ) เป็นต้นไป`  
`เพื่อนำไปคำนวณต่อ ก็สามารถใช้การ slices เพื่อตัดแค่ข้อมูลที่ต้องการ แล้วนำมาใช้ต่อได้เลย`  


---

## 4. Key Concepts

### 4.1 `[Memory Model ของ Stack]`

**คำอธิบาย**

`[Stack สำหรับเก็บค่าที่เป็น Imutable หรือค่าที่ถูกฟิคขนาดไว้แล้ว ในช่วงเวลานั้นๆ]`

**ตัวอย่าง**

```rust
fn main(){
    let x = 60;
    let y = 7;
    let z = x + y;
}
```

**Explanation**

`[ตัว Memory Model ของภาษา rust จะเก็บค่า x และ y เข้าไปใน stack ก่อน และให้มันมีค่าเป็น 60 และ 7 ตามลำดับ]`  
`[จากนั้น Memory ของภาษา rust จะ อ่านค่า x และ y นำมาบวกกันแล้วเก็บเข้าไปในค่า z ของ stack]`  


---

### 4.2 `[Memory Model ของ Heap]`

`[Heap จะเป็นก้อนเก็บข้อมูลก้อนนึง สำหรับตัวแปรที่ยืดหยุ่นเรื่องขนาดระหว่างการคอมไพล์ โดยตัวแปรเหล่านั้นจะมีทั้งเก็บค่าไว้ที่ stack และ heap]`
`[โดยหลักๆจะแบ่งเป็น 2 ประเภท 1.เก็บค่าใน stack เป็น thin pointer , 2.เก็บค่าใน stack เป็น fat pointer เพื่อชี้ข้อมูลไปที่ ก้อนใน heap]`

```rust
fn main(){
    let s = String::from("Hello");
    let myVec = vec![1, 2, 3, 5];
}
```
**Explanation**

`เริ่มที่ตัว s จะสร้างก้อนheap ที่เก็บคำว่า ['H' , 'e' , 'l' , 'l' , 'o']ไว้ แล้วจะเก็บค่าใน stack เป็น pointer + len + capacity `
`โดย pointer จะชี้ไปที่ก้อน heap`  
`ส่วน myVec ก็จะทำงานในทำนองเดียวกัน`  

---

### 4.3 `[Slicing]`

`[อธิบายแนวคิด]`

```rust
fn main() {
    let s = String::from(“Silpakorn”);
    
    // แบบที่ 1 จะได้ตัวเอง
    let same = &s[..];
    println!("Slices ได้ตัวเอง จะได้ {same}");

    // ควรระวัง หากกำหนดเอง เนื่องจากการ Slice จะ Slice ถึงแค่ n-1
    // แบบที่ 2 ตั้งแต่ตัวแรกถึงตัวที่เรากำหนด
    let silp = &s[..3];
    println!("Slices ถึงตัวที่ 3 จะได้ {silp}");

    // 3 ตั้งแต่ตัวที่ i ถึงตัวที่ n ซึ่ง i กับ n เราสามารถกำหนดเองได้
    let pako = &s[3..7];
    println!("Slices ตั้งแต่ตัวที่ 3 - 7 จะได้ {pako}");

    // 4 ตั้งแต่ตัวที่เรากำหนดเป็นต้นไป
    let rn = &s[7..];
    println!("Slices ตั้งแต่ตัวที่ 7 เป็นต้นไป จะได้ {rn}");

}
```

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |

### Important Rules

1. `[กฎสำคัญข้อที่ 1]`
2. `[กฎสำคัญข้อที่ 2]`
3. `[กฎสำคัญข้อที่ 3]`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code ทีละส่วนที่สำคัญ]`

---

### Example 2 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code]`

---

## 7. Common Mistakes

### Mistake 1 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

### Mistake 2 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

### Exercise 2 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`[Topic นี้เกี่ยวข้องกับ syntax อย่างไร]`

### 9.2 Semantics

`[คำสั่ง/construct เหล่านี้มีความหมายหรือพฤติกรรมอย่างไร]`

### 9.3 Type System

`[เกี่ยวข้องกับ type system อย่างไร ถ้ามี]`

### 9.4 Memory / Resource Management

`[เกี่ยวข้องกับ memory หรือ resource management อย่างไร ถ้ามี]`

### 9.5 Abstraction / Other PPL Concepts

`[อธิบาย abstraction, scope, binding, paradigm หรือแนวคิด PPL อื่นที่เกี่ยวข้อง]`

### 9.6 Why Rust?

`[Rust ใช้แนวคิดนี้เพื่อเพิ่ม safety, reliability หรือ performance อย่างไร]`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `[อธิบาย]` | `[อธิบาย]` |
| Semantics / Behavior | `[อธิบาย]` | `[อธิบาย]` |
| Type System | `[อธิบาย]` | `[อธิบาย]` |
| Memory Management | `[อธิบาย]` | `[อธิบาย]` |
| Safety | `[อธิบาย]` | `[อธิบาย]` |

### Rust Example

```rust
// Rust code
```

### `[Other Language]` Example

```python
# Other language code
```

### Analysis

`[อธิบายความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษา]`

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 | Concept + Short Code Illustration | 5 min |
| Member 2 | Detailed Code + Live Demo | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`[สิ่งที่รับผิดชอบ]`

**Member 2**

`[สิ่งที่รับผิดชอบ]`

**Member 3**

`[สิ่งที่รับผิดชอบ]`

**Member 4**

`[สิ่งที่รับผิดชอบ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[แหล่งอ้างอิงเพิ่มเติม]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `[Gemini Pro]` | `[การคอนเฟิร์มในส่วนของ Memory model]` | `[โดยไปดูแหล่งอ้างอิงมา แล้วนำมาแชทกับ AI อีกที]` |
| `[เช่น ChatGPT]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |
| `[AI tool]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 2 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 3 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 4 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`[ในวันที่ 24/9/69 : ได้มีการประชุมเพื่อตกลงกันในเรื่องเนื้อหา เนื่องจาก เลยหารือแนวทางร่วมกัน และนัดวันที่ในการเดดไลน์ Rust_tutorial_template + ตรวจสอบโค้ดที่รันได้ทั้งหมด ในวันเสาร์ที่ 26/9/68 พร้อมเริ่มทำสไลด์ในการ Presentation]`

**Problems encountered**

`1. [ตัวโค้ดที่ลง ระหว่างของสมาชิกคนที่ 1 และสมาชิกคนที่ 2 อาจะเกิดการทับซ้อนกัน]`

**How did you solve them?**

`1. [หารือและแบ่งได้ว่า สมาชิกคนแรกอาจทำแค่ slice List และ String เบื้องต้น ส่วนสมาชิกคนที่สองอาจทำเป็น slice word หรือหา sliceส่วนอื่นที่ลงลึกกว่านี้]`

---

## 15. Final Checklist

- [ ] Learning Objectives ครบ 3–4 ข้อ
- [ ] Key Concepts ครบถ้วน
- [ ] Syntax / Rules
- [ ] Runnable Code Examples
- [ ] Code Compile และ Run ได้จริง
- [ ] Common Mistakes
- [ ] Exercises 2 ข้อ พร้อม Solutions
- [ ] PPL Perspective
- [ ] Rust vs Other Language
- [ ] References อย่างน้อย 4 แหล่ง
- [ ] AI Usage Declaration
- [ ] GitHub Contribution
- [ ] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `[https://github.com/670710294/rust_slices_and_memory_model]`

**Chapter Path:** `[เช่น chapters/01-introduction/]`

**Final PR:** `#[PR number]`

**Submitted by:** `[Group XX]`

**Date:** `[YYYY-MM-DD]`
