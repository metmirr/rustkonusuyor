///
/// v1.iter() geriye bir iterator gönderir. Bu iterator bir eleman
/// dizisi olduğundan map() fonkisyonu her elemen için çalışır ve 
/// map içerisinde yapılan işlemi her elemana uygular(örneğimizde 
/// her elemana bir ekliyoruz.)
/// Son olarak collect() fonksiyonunu çağırıyoruz, bu fonksiyon geriye
/// bir vektor gönderir. Bu vektor en son olarak 2,3,4 değerlerine
/// sahip bir dizi olur. 
/// 
/// collect() fonkisyonu bir iteratorü tüketir. Tüketmeden maksat 
/// kullanır diyebiliriz.
///
/// trait: Rust programlama dilindeki trait yapısı object-oriented
/// dillerdeki interface yapısına çok benzerdir. Bir kaç farklı özelliği
/// olsada genel anlamda kullanım amacı, trait rust derleyicisine bir veri
/// tipinin bir özelliği sağlaması gerektiğini söyler. Örneğin bir trait
/// içerisinde bir metot tanımlanmışsa, bu trait'i implement(uygulayan) veri
/// tipi bu metodu kendi içerisinde tanımlamak zorundadır. aksi halde derleyici
/// hata verir.
/// trait içerisinde tanımlanan metotlar gövdesiz olarak tanımlanır.
///
/// Şimdi rust ile gelen Iterator trait'ini kullanarak bir iterator oluşturalım. Bu
/// 



trait Iterator {
	type Item;
	fn next(&mut self) -> Option<Self::Item>;
 }


struct Counter {
    count: u32
}

impl Counter {
	fn new() -> Counter {
	    Counter{count: 0}
	}
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
    	self.count += 1;
    	if self.count < 6 {
    		Some(self.count)
    	} else {
    	    None
    	}
    }
}


fn main() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v2, [2, 3, 4]);


    let mut counter = Counter::new();
    let x = counter.next();
    println!("{:?}", x);
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
	println!("{:?}", counter.next());
	println!("{:?}", counter.next());
}