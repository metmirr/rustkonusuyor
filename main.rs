/// Değişken tanımlama:
///   - Değişken tanımlarken `let` anahtar kelimesi kullanılır. Değerleri 
///   değiştirilebilir değişkenler `let mut` şeklinde tanımlanmalıdır(mut : mutable ).
///   - Değişken değerleri ekrana yazdırılırken tırnaklar arasına {} işaretinin eklenmesi gerekir.
///   aksi halde değişken değeri ekrana yazdırılmaz.

fn main() {
  let x = 100;
  //x = 200; //hata! 
  let mut y = 300;
  //y = 400; //çalışır
  
  // x değişkeninin değerini ekrana yazdırma.
  println!("{}", x);
}

