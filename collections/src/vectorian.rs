pub mod col {
  pub fn list() {
    let v = vec![100, 32, 57];
    for i in &v {
      println!("{}", i);
    }
  }

  pub fn inc() {
    let mut v = vec![100, 22, 32];
    for i in &mut v {
      *i += 50
    }

    println!("{:?}", v);
  }

  pub fn spread() {
    enum Spreadsheetcell {
      Int(i32),
      Float(f64),
      Text(String),
    }

    let _row = vec![
      Spreadsheetcell::Int(3),
      Spreadsheetcell::Text(String::from("blue")),
      Spreadsheetcell::Float(10.12),
    ];
  }
}
