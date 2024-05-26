fn main() {
    println!("Hello, world!");
}

#[test]
fn main_test(){
    println!("Hello, World!");
    println!("Hello, World!");
    println!("Hello, World!");
}

#[test]
fn variable_test(){
    let name = "Ricky";
    let umur = 12;
    println!("Halo, nama saya {}. umur saya {}", name, umur);
}

#[test]
fn variable_mut_test(){
    let mut name = "Ricky";
    let umur = 22;
    println!("Halo, nama saya {}. umur saya {} tahun.", name, umur);

    name = "Kasino";
    println!("Halo, nama saya {}. umur saya {} tahun.", name, umur);
}

#[test]
fn static_typing(){
    /*
    Compiling helloworld v0.1.0 (/home/ricky/Desktop/ngoding/helloworld)
    warning: variable does not need to be mutable
    --> src/main.rs:31:9
    |
    31 |     let mut name = "Ricky";
    |         ----^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default
     */

    // variabel di bawah tidak akan error, tapi akan menghasilkan warning jika variabel name tidak perlu di mut karena tidak pernah diubah
    // hal ini yang memaksa kita untuk melakukan pengodean secara disiplin/clean code
    // let mut name = "Ricky";
    let name = "Ricky";
    let umur = 22;
    println!("Halo, nama saya {}. umur saya {} tahun.", name, umur);

    // name = "Ricky";
}

#[test]
fn shadowing(){
    // shadowing adalah kondisi di mana variabel yang sama diperbarui, tapi bukan mutable
    // hal ini berhubungan dengan penyimpanan memory (perbedaan address)
    let name = "Ricky";
    let umur = 22;
    println!("Halo, nama saya {}. umur saya {} tahun.", name, umur);

    // di sini, kita mendeklarasikan variabel baru karena terdapat let di depannya
    // secara teori, ketika mendeklarasikan variable baru (terdapat keyword let), maka value dalam memory tidak berubah, melainkan menyimpannya pada address baru
    // artinya, value yang tersimpan pada memory "Ricky", itu berbeda dengan address yang menyimpan "Kasino" walaupun variabelnya sama
    let name = "Kasino";
    println!("Halo, nama saya {}. umur saya {} tahun.", name, umur);
}

#[test]
fn explicit(){
    // secara default, rust akan secara otomatis mendefinisikan type data
    // namun, kita bisa juga mendefinisikannya sendiri (tidak wajib)
    let age: i32 = 22;
    let name: &str = "Rick";
    println!("Halo, nama saya {}. umur saya {} tahun",name, age);
}

#[test]
fn boolean_test(){
    // default boolean
    let a = true;

    // diatur sendiri secara explicit juga bisa
    let b: bool = false;
    println!("{} - {}", a, b);
}

#[test]
fn comparison_test(){
    // default boolean
    let a = 10;

    // diatur sendiri secara explicit juga bisa
    let b = 20;

    let c = a > b;

    println!("{}", c);
}

#[test]
fn boolean_operator(){
    let nilai_uts = 80;
    let nilai_uas = 75;

    let lulus_uts = nilai_uts >= 75;
    let lulus_uas = nilai_uas >= 75;

    let lulus_ujian = lulus_uts && lulus_uas;

    println!("{}", lulus_ujian);
}

#[test]
fn tuple(){
    let tuple: (i8, &str, bool) = (32,"Ricky",true);

    // cara print tuple dengan cara {:?}
    println!("{:?}", tuple);
}

#[test]
fn access_tuple(){
    let tuple: (i8, &str, bool) = (32,"Ricky",true);

    // cara mengakses nilai tuple adalah dengan variabel_tuple.index (index tuple dimulai dari 0)
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;
    println!("{} - {} - {}", a,b,c);
}

#[test]
fn destructuring_tuple(){
    let tuple: (i8, &str, bool) = (32,"Ricky",true);

    // cara melakukan destructuring seperti ini
    let (a,b,c) = tuple;
    
    // jika ada data yang tidak dibutuhkan, dapat menggunakan _
    let (d, e, _) = tuple;
    println!("{} - {} - {}", a,b,c);
    println!("{} - {}", d,e);
}

#[test]
fn mutable_tuple(){
    // secara default, tuple itu imutable (tidak bisa dirubah valuenya)
    // namun, variabel tuple bisa dijadikan mutable dengan cara menambahkan keyword 'mut'

    let mut data: (i8, &str, bool) = (10, "Rick", true);
    
    // cara melakukan destructuring seperti ini
    let (a,b,c) = data;

    println!("{} - {} - {}", a,b,c);

    /*
        cara mengubah valuenya sama seperti cara mengakses data tuple
        variabel_tuple.index = value;

        ingat! tipe datanya harus sama
     */
    data.0 = 9;
    data.1 = "Chris";
    data.2 = false;

    let (d,e,f) = data;
    println!("{} - {} - {}", d,e,f);
}

fn unit(){
    println!("Hello");
}

#[test]
fn tuple_kosong(){
    /* 
    di rust, kita bisa membuat sebuah tuple kosong
    biasanya digunakan untuk fungsi fungsi
    */ 
    let tuple_kosong = unit();
    println!("{:?}", tuple_kosong);

    let tuple_kosong_1: () = ();
    println!("{:?}", tuple_kosong_1);
}

#[test]
fn array(){
    /*
    - array adalah tipe data yang mirip seperti tuple
    - bedanya, array hanya boleh berisi 1 tipe data saja
    - kalau integer ya integer semua
    - berbeda dengan javascript dan php, panjang array di rust harus fix (seperti golang dan java)
    - struktur => let nama_variabel: [tipe data: panjang data] = [data]
    - secara default bisa dideklarasikan seperti ini => let nama_variabel = [data]
     */

    let array: [i8; 5] = [1,2,3,4,5];
    println!("{:?}", array);

    let array_1 = [1,2,3];
    println!("{:?}", array_1);

    /*
    - cara mengakses array hampir sama seperti tuple, bedanya ketika tuple menggunakan tititk variabel_tuple.index kalau array menggunakan variabel_array[index]
     */
    let a = array[0];
    let b = array[1];
    println!("array index 0 adalah : {}, array index 1 adalah : {}", a, b);

    /*
    secara default, array itu imutable
    tapi bisa kita ubah menjadi mutable dengan keyword 'mut'
    kemudian, kita bisa ubah isi tuplenya dengan cara 
    variabel_tuple[index] = value;
     */
    let mut array_2 = [1,2,3];
    println!("array sebelum diubah : {:?}", array_2);
    array_2[0] = 10;
    array_2[1] = 30;
    println!("array sesudah diubah : {:?}", array_2);

    /*
    salah satu pembeda antara tuple dengan array adalah kita bisa mendapat panjang dari array (length nya)
     */

    let c = array_2.len();
    println!("panjang dari array_2 adalah : {:?}", c);
}

#[test]
fn two_dimensional_array(){
    /*
    terkadang, dalam sebuah case kita akan mendapatkan kondisi di mana kita harus membuat array di dalam array
    rust dapat melakukan hal tersebut

    cara deklarasi dan assignment nya seperti ini
    variabel_array: [[tipe_data;panjang array lapisan 2], panjang array lapisan 1]
     */

    let array_2d: [[i32; 2]; 3] = [[1,2], [3,4], [1,3]];

    println!("{:?}", array_2d);
    println!("{:?}", array_2d[0]);
    println!("{:?}", array_2d[1]);
    println!("index 0 - 0 : {}",array_2d[0][0]);
    println!("index 0 - 1 : {}",array_2d[0][1]);
    println!("index 1 - 0 : {}",array_2d[1][0]);
    println!("index 1 - 1 : {}",array_2d[1][1]);
}

#[test]
fn constant(){
    /*
    - konstan adalah variabel yang tidak akan berubah nilainya
    - nilai dari sebuah konstan harus didefinisikan saat program ditulis, bukan dijalankan
    - constant tidak sama dengan let, karena constan tidak memiliki keyword 'mut'
    - konstan harus didefinisikan tipe datanya secara explicit
    - cara mendeklarasikannya seperti ini :
        const NAMA_DENGAN_CAPITAL: tipe_data = value;
     */

    const MINIMUM: i32 = 8;
    println!("{}", MINIMUM);
}