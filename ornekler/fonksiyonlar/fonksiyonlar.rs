// Her rust programi main() fonksiyonunu bulundurmak zorundadir.
// Fonksiyonlar fn anahtar kelimesi ile tanimlanirlar. 
// Parametreler parantezlerin arasina yazilir ve parametrelerin tipleri belirtilmek zorundadir.
// Varsa fonksiyondan geriye dondurulecek degerin tipi belirtilmelidir( -> okdan sonra):
//   
//   fn fonk_adi(para1: tip, para2: tip) -> geri_donus_degeri { }
//
// Rust'da bir fonksiyonadan geriye bir deger donderilecekse bu deger fonksiyonda en son satir olarak
// yazilip noktali virgul konulmazsa return kelimesine gerek yoktur. Cogunlukla kullanilan bir 
// tasarimdir.
//
// Fonksiyonlar degiskenlere atanan degerler gibi kullanilabilirler
// 
//   let f: f();
//
// Bu sekilde tanimlanan bir degiskene ancak bir fonksiyon atanabilir, cunku degiskenin tipi 
// fonksiyon tipindedir, ve atanan fonksiyonun imzasi(parametreler, geri donus degeri) uyusmalidir.


fn main() {
	let x = toplama(1, 3);
	println!("{}", x);
	println!("{}", carpma(4, 5));
	bolme(5.0, 9.0);

    let f: fn(); // parametresiz, geri donus degeri yok
    let f1: fn(i32, f32);

    let f3: fn(&str) = sayhello;
    f3("Suleyman");

}

fn toplama(a: i32, b: i32) -> i32 {
	a + b // geri donus degeri
}

fn carpma(a: i32, b: i32) -> i32 {
	return a * b;
}

fn bolme(a: f32, b: f32) {
	println!("{}", (a / b));
}

fn sayhello(name: &str) {
    println!("Merhaba, {}!", name);
}

