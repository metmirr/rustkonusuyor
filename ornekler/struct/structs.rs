// struct özel bir veri tipidir. Birbiriyle ilişkili birçok veriyi
// bir arada tutmak için kullanılır. Eğer nesne tabanlı bir programlama
// dilinden geliyorsanız, structları nesneler gibi düşünebilirsiniz.
//
// struct'ların alanları(fields) olabileceği gibi alan belirtilmeden de struct 
// tanımlamak mümkündür.
// 
// structlar için metotlar tanımlanabilir. Bu metotları tanımlarken:
//
// impl StructAdi { fn metot_adi(parametre_varsa: veri_tipi) varsa_geridönüş_degeri{}}
//
// impl bloğu içerisinde birden fazla metot tanımlanabilir.
//
// Bu metotlar eğer aşağıdaki parametrelerden birini alıyorsa, metotlara erişmek
// için `.` kullanılır: 
// 
// parametreler:
// 
//		&self
//		&mut self
// 
// Bu parametrelerden &self struct'ın kendisidir. Örneği self.username, self kelimesi
// User struct'ıdır. Rust bu parametreyi özel olarak kendisini işaret ettiği manasında
// self olarak isimlendirir. &mut self, parametresi ise o structın alanlarındaki  
// değerlerin değiştirilebildiğini işaret eder. mut kelimesi mutable kelimesinin kısaltımıdır.
// 
// Eğer tanımlanan metotlar bu parametrelerden birini almıyorsa o metodu çağırmak içi `::`
// kullanılır: StructAdi::metot_ismi();
//
// Son olarak structları ekrana yazdırmak için Debug struct'ından türetiyoruz. Diğer
// türlü println!("{}", struct_adi) şeklinde bir kullanım hata mesajı verecektir. Debug
// amaçlı bu şekilde türetme yayet yararlı bir kullanımdır.
//

#[derive(Debug)]
struct User {
	username: String,
	email: String,
	age: u32 
}

impl User {
	fn new() -> User {
		User{ 
			username: String::from("New User"), 
			email: String::from("newuser@mail.com"), 
			age: 0
		}
	}

	fn edit_user(&mut self, uname: String, email: String, age: u32) {
		self.username = uname;
		self.email = email;
		self.age = age;
	}
}

fn main() {
	let mut user = User::new();
	
	println!("{:?}", user);
	
	user.edit_user(String::from("Kal"), String::from("Be"), 0);
	
	println!("{:?}", user);
}
