// Copyright © 2019 Jeff Austin, Kamakshi Nagar
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rand;
use rand::Rng;

// return a random even number <= the passed parameter
pub fn rand_even(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut rng = rand::thread_rng();

    // using a trick for getting an even such as performing integer division and then returning
    // `num` multiplied by 2 does not work here because in the case of the lower bound being 1,
    // 1 / 2 would be zero, and we could end up with a value of 0 as a base/size. So we use modulo.

    loop { // looping forever
        let num: u32 = rng.gen_range(lower_bound, upper_bound);
        if num % 2 == 0 { // => the number is even
            return num;
        }
    }
}

        //rng = rand::thread_rng();
// return a random power of 2 (u32)
pub fn rand_power_of_2(lower: u32, upper: u32) -> u32 {
    let mut found = false;
    let mut dummy: u32 = 2;

    let mut rng = rand::thread_rng();
    while !found {
        let num = rng.gen_range(lower - 1, upper + 1);  // ensure that the lower and upper bounds are included in the search domain.
        let mut done = false;  // not done multiplying
        while !done {
            while dummy < num {
                dummy *= 2;
            }
            if dummy == num {
                found = true;
                done = true;  // its a power of 2 --success!
            }
            else if dummy > num {
                done = true;  // its not a power of 2 and we need to generate again
            }
        }  // done multiplying
        if done && found {
            return num;
        }
        dummy = 2;
        found = false;
    }
    0
}
