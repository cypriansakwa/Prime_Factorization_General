fn main() {
    let number = 31500; // The number to be factored
    let factors = prime_factors(number);
    
    println!("Prime factors of {} are: {:?}", number, factors);
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    
    //A while loop tests if n is divisible by two.
    //If n is divisible by two, it adds two to the factor vector and divides it by two. 
    //This loop repeats until n is no longer divisible by two.
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    // n must be odd at this point, so we can skip even numbers
    //The code then proceeds to check odd factors beginning with 3.
    //A while loop iterates with i beginning at 3 and increasing by 
    //2 with each iteration (skipping even values).
    //A while loop determines if n is divisible by i for each i. 
    //If so, n is divided by i and i is added to the factors vector. 
    //This keeps happening until n can no longer be divided by i.
    let mut i = 3;
    while i * i <= n { 
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }

    // In the event that n remains more than 2 after all, it indicates 
    //that n is a prime number that is added to the factors vector.
    if n > 2 {
        factors.push(n);
    }

    factors
}
