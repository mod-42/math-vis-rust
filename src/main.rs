 fn main() {
    
    let cycl = 7;
	let mut vect1 = vec![0,];
	let mut vect2 = vec![2,];
	let mut eul = vec![2,];
	
	/*filling of vect1 with the Numbers 2 to m and vect2 with 
	the number 1*/

	for i in 0..(cycl - 1) {
		
		vect1.push(i+2);		
		vect2.push(1);

	} 
	
	for i in 0..6 {
		
		let mut c = 0;
		let mut d = 0;
		let mut e = 0;
		let mut k = 6;

		for j in 0..6 {

			c = vect2[k] * 10 + d;

			e = c % vect1[k];
			
			d = c / vect1[k];

		    vect2[k] = e;

			k -= 1;

		}
		
		eul.push(d);
	}

	for i in 0..cycl {
		
		println!("{}", eul[i]);

	}

}

//#############################################################################################################

/*
//approximation for e

fn name(arg: Type) -> RetType {
	
	let mut eul = 1.0;
	let mut approx = 0.0;
	let mut step = 1.0;

	for divi in 1..14 {
				
		let divin = divi as f64;

		step = divin*step;
		eul = eul+approx;
		approx = 1.0/ step;

	}

	println!("Wert von e {}", eul );
}
*/

//###############################################################################################################



/*

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
