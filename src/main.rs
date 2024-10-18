fn main() {
    println!("Hello world! Hi!");
    println!("Wassup");
    let sayi1=17;

    let decimal:f64=12.2;

    let tek :char='a';
    let tek ='d';
    let tek ='a';

    let toplam = "ada";

    
    

    calculator(13, sayi1, 3);

}


fn toplama(number:i32,number1:i32) {

    if number<number1 {
        println!("yanlış")

    }
    else {
    let toplam = number+number1;

    println!("Toplamanın sonucu {}",toplam);
    }
    
}


fn cikarma (number:i32,number1:i32) {

    if number<number1 {

        println!("İlk girdiğiniz sayı ikinci sayıdan küçük bu yüzden olmaz")
        
    }
    else {
        let sonuc=number-number1;
    println!("cikarmanin sonucu {}", sonuc);
        
    }

    
}

fn calculator(number:i32,number1:i32,değer:i32){

    if değer ==1 {

        println!("toplamanın sonucu :{}",number+number1)
        
    }
    else if değer ==2 {

        println!("çıkarmanın sonucu :{}",number-number1)
        
    }
    else if  değer ==3 {

        println!("çarpmanın  sonucu :{}",number*number1)
        
    }
    else if değer ==4 {

        println!("bölmenin sonucu :{}",number/number1)
        
    }
    else {
        println!("Geçersiz değer")
    }

}


