use std::env;
// CSC318 Coursework 1

// ----------Part 1----------

// 1: "JONATHAN WOOLLETT LIGHT WHY JUST WHY" -> "OTSFYMFS CTTQQJYY QNLMY CME OZXY CME"
//     Can't actually encrypt my name since it has a '-' due to "Woollett-Light" but since to
//      decrypt the message in q2 this is outside the range, I guess this is just how it goes.

// 2: "L KDWH LW ZKHQ SHRSOH DUH DW BRXU KRXVH DQG DVN GR BRX KDYH D EDWKURRP" -> 
//    "I HATE IT WHEN PEOPLE ARE AT YOUR HOUSE AND ASK DO YOU HAVE A BATHROOM"

// 3: It is incredibly easy to crack ciphertext encrypted with a shift cipher.
//    A larger range linearly increases how hard it is to crack.
//    We can simply brute force it, by shifting through the entire range.
//    Cracking a shift cipher is O(n).

// Due to how you weirdly encoded the string (not encoding spaces) you ask us to decode.
// When ever a space is encountered it requires a special condition to ignore it.
// I beleive a proper shift cipher should encode the whole sentence including spaces 
// across the whole ASCII range.
// So you have decided the range should be 65->90 (inclusive) with an exception for spaces.
// Kinda annoying I needed to infer this range, also don't get why you choose it, anyway
// you can adjust it using the below constants if you want.

const SHIFT_MAX:u8 = 90u8;
const SHIFT_MIN:u8 = 65u8;
const SHIFT_DIFF:u8 = SHIFT_MAX - SHIFT_MIN;

// ----------Part 1----------

// 4: "true friends do not judge each other but they judge other people together" -> 
//    "tufinsoojdeahtebthyugohrepeoehrrereddntugecohrutejdetepoltgte"

// Once again the fact we have to ignore spaces makes the whole thing rather more annoying and
// inelegant to program.

fn main() {
    // Presumes ASCII input (range within which varies)
    let arguments:Vec<String> = env::args().collect();
    let input:&[u8] = arguments[1].as_bytes();
    //println!("{:.?}",input);
    
    // let shift_cipher_encoded = shift_cipher_encode(input,5);
    // println!("{}",String::from_utf8(shift_cipher_encoded.clone()).unwrap());

    // let shift_cipher_decoded= shift_cipher_decode(input,3);
    // println!("{}",String::from_utf8(shift_cipher_decoded).unwrap());

    // let rail_fence_cipher_encoded = rail_fence_cipher_encode(input, 2);
    // println!("rail_fence_cipher_encoded: {}",String::from_utf8(rail_fence_cipher_encoded.clone()).unwrap());
    // let rail_fence_cipher_decoded = rail_fence_cipher_decode(&rail_fence_cipher_encoded, 2);
    // println!("rail_fence_cipher_decoded: {}",String::from_utf8(rail_fence_cipher_decoded).unwrap());

    // println!("\n\n\n\n\n");

    // let rail_fence_cipher_decoded = rail_fence_cipher_decode(input, 3);
    // println!("rail_fence_cipher_decoded: {}",String::from_utf8(rail_fence_cipher_decoded.clone()).unwrap());
    // let rail_fence_cipher_encoded = rail_fence_cipher_encode(&rail_fence_cipher_decoded, 3);
    // println!("rail_fence_cipher_encoded: {}",String::from_utf8(rail_fence_cipher_encoded).unwrap());
}

// `#[allow(dead_code)]` just gets rid of dead code compilation warning

#[allow(dead_code)] fn shift_cipher_encode(value:&[u8],key:u8) -> Vec<u8> {
    let mut new_value = Vec::with_capacity(value.len());
    for val in value {
        if *val == 32u8 { // exception for spaces
            new_value.push(32u8); 
        }
        else {
            let value_plus_key = val + key;
            new_value.push(
                if value_plus_key > SHIFT_MAX { value_plus_key - SHIFT_DIFF } 
                else { value_plus_key }
            );
        }
    }
    return new_value;
}
#[allow(dead_code)] fn shift_cipher_decode(value:&[u8],key:u8) -> Vec<u8> {
    let mut new_value:Vec<u8> = Vec::with_capacity(value.len());
    for val in value {
        if *val == 32u8 { // exception for spaces
            new_value.push(32u8);
        }
        else {
            let value_minus_key: i16 = *val as i16 - key as i16;
            //println!("value_minus_key: {}", value_minus_key);
            new_value.push(
                // This is 32 since in part 1 question 2 it asks me to decode a string
                //  decoding this string strictly according to ASCII turns 'space' chars 
                //  into 'group seperator' chars, this is clearly not desired.
                if value_minus_key < SHIFT_MIN as i16 { (value_minus_key + SHIFT_DIFF as i16 + 1i16) as u8 } 
                else { value_minus_key as u8 }
            );
        }
        
    }
    return new_value;
}

#[allow(dead_code)] fn rail_fence_cipher_encode(value:&[u8],num_rails:usize) -> Vec<u8> {
    // Removes spaces
    let mut value_without_spaces:Vec<u8> = Vec::new();
    for i in 0..value.len() {
        if value[i] != 32 /*32=space*/ { value_without_spaces.push(value[i]) }
    }
    

    let mut rails:Vec<Vec<u8>> = vec![Vec::with_capacity(value_without_spaces.len() / num_rails);num_rails];
    let mut current_rail:i8 = 0;
    let mut rail_change: i8 = 1;
    for i in 0..value_without_spaces.len() {
        rails[current_rail as usize].push(value_without_spaces[i]);
        // If `0` or `num_rails`, flip `rail_change`. Code could be compressed, but reduces readability.
        current_rail += rail_change;
        if current_rail == 0 { rail_change = 1; } else if current_rail + 1i8 == num_rails as i8 { rail_change = -1; }
    }
    //Prints rails contents
    for i in 0..rails.len() {
        println!("rail[{}]: {}",i,String::from_utf8(rails[i].clone()).unwrap());
    }
    return rails.into_iter().flatten().collect();
}

#[allow(dead_code)] fn rail_fence_cipher_decode(value:&[u8],num_rails:usize) -> Vec<u8> {
    // Removes asterisks(*) and dashes(-)
    let mut filtered_value:Vec<u8> = Vec::new();
    for i in 0..value.len() {
        // if !(value[i]==42 /*42=`*`*/ || value[i]==45 /*45=`-`*/) { 
        //     filtered_value.push(value[i]);
        // }
        filtered_value.push(value[i]);
    }
    println!("filtered_value: {}",String::from_utf8(filtered_value.clone()).unwrap());

    // Construct rails of right lengths
    let mut rails:Vec<Vec<u8>> = vec![Vec::new();num_rails];
    let mut current_rail:i8 = 0;
    let mut rail_change: i8 = 1;
    for _ in 0..filtered_value.len() {
        rails[current_rail as usize].push(0u8);
        // If `0` or `num_rails`, flip `rail_change`. Code could be compressed, but reduces readability.
        current_rail += rail_change;
        if current_rail == 0 { rail_change = 1; } else if current_rail + 1i8 == num_rails as i8 { rail_change = -1; }
    }

    // Push onto rails
    let mut p = 0usize;
    for i in 0..rails.len() {
        for t in 0..rails[i].len() {
            rails[i][t] = filtered_value[p];
            p += 1;
        }
    }

    for i in 0..rails.len() {
        println!("rail[{}]: {}",i,String::from_utf8(rails[i].clone()).unwrap());
    }


    // Pull off of rails
    let mut decoded:Vec<u8> = Vec::with_capacity(filtered_value.len());
    current_rail = 0;
    rail_change = 1;
    for _ in 0..filtered_value.len() {
        decoded.push(rails[current_rail as usize].remove(0));
        // If `0` or `num_rails`, flip `rail_change`. Code could be compressed, but reduces readability.
        current_rail += rail_change;
        if current_rail == 0 { rail_change = 1; } else if current_rail + 1i8 == num_rails as i8 { rail_change = -1; }
    }

    //Prints rails contents
    
    return decoded;
}