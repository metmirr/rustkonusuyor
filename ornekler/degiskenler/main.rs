//
// Rust'da Degiskenler let anahtar kelimesi ile tanimlanir
// Degiskenin tipini belirtmezseniz rust'in tip sistemi bunu derleme zamaninda belirler.
// Degisken tipi Degisken isminden sonra : eklenerek belirtilebilir:
// 
//   let x: i32 = 45;
//
// Birden cok Degisken tek seferde tanimlanabilir:
//
//   let (a, b) = (9, 13);
//
// Rust Degiskenleri varasayilan olarak unmutable(degistirilemez) olarak tanimlar. Yani bir Degiskene
// deger atadiginizda hemen sonrasinda degerini degistirirseniz derleme zamaninda hata alirsiniz. Rust bunu
// guvenlik amaciyla yaptigini soyler:
// 
//   let x = 45;
//   x = 0; // hata!
//
// Bir Degiskenin degerini degistirmek istiyorsaniz bu Degiskeni mutable(degistirilebilir) manasinda `mut` anahtar
// kelimesiyle tanimlamalisiniz:
//
//   let mut x = 45;
//   x = 0; // sorun yok
//

fn main() {
    let x = 45;
    println!("{}", x);

    let (a, b) = (9, 13);
    println!("a: {}, b: {}", a, b);

    let mut y = 3;
    println!("{}", y);
}