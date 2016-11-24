// Fonksiyonlar degiskenlere atanan degerler gibi kullanilabilirler
// 
//   let f: f();
//
// Bu sekilde tanimlanan bir degiskene ancak bir fonksiyon atanabilir, cunku degiskenin tipi 
// fonksiyon tipindedir, ve atanan fonksiyonun imzasi(parametreler, geri donus degeri) uyusmalidir.

fn main() {
	let f: fn(); // parametresiz, geri donus degeri yok
	let f1: fn(i32, f32);

	let f3: fn(&str) = sayhello;
	f3("Suleyman");
}

fn sayhello(name: &str) {
	 println!("Merhaba, {}!", name);
}