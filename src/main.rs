fn main() {

println!("Programm zu prüfung von Kommutativ- Distriputiv- und Kommutativ Gesetz!");     
println!("Abschnitt zur Prüfung des Kommutativ Gesestz der Addition für x Element N 0 bis 100");

let mut errors = 0;
for x in 0..101 {
  for y in 0..101 {
    if x+y != y + x {
      errors +=1;
    }
  }
}


println!("number o errors: {} ",errors);

	
}


/*

	
*/
/*
fm kommutativ-addition(){

	let mut a = 0;	// a: i32
	let mut b = 101;
	let mut c = 0;
	let mut d = 0;

	a+b=b+a
	
}
fm distributiv(){ 
	a*(b+c)=a*b+a*c
	(a+b)*c=a*c+b*c
}

fm asossativ

a*(b*C)=(a*b)*c
*/
