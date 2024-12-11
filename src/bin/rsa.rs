// src/bin/rsa.rs

use rand::Rng;

pub struct PubKey {
    modulus: i64,
    exponent: i64,
}

pub struct PriKey {
    modulus: i64,
    exponent: i64,
}


pub fn is_prime(n: i64) -> bool {
   for i in 2..n/2 {
       if n % i == 0 {
           return false;
       }
   }
   true
}

pub fn gen_prime() -> i64 {
    let mut rng = rand::thread_rng();

    let mut random_number: i64 = 10000 + rng.gen_range(0..10000);
    while !is_prime(random_number) {
        random_number += 1;
    }
    random_number
}

pub fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

pub fn ext_euclid(mut a: i64, mut b: i64) -> i64 {
    //let mut x = 0, y = 1, u = 1, v = 0, gcd = b, m, n, q, r;
    let (mut x, mut y, mut u, mut v, mut gcd, mut m, mut n, mut q, mut r) = (0, 1, 1, 0, b, 0, 0, 0, 0);
    while a != 0 {
        q = gcd / a;
        r = gcd % a;
        m = x - u * q;
        n = y - v * q;
        gcd = a;
        a = r;
        x = u;
        y = v;
        u = m;
        v = n;
    }
    y
}

pub fn gen_keys () -> (PubKey, PriKey) {
    let mut p = gen_prime();
    let mut q = gen_prime();
    let e = (2 << 16) + 1;
    let mut max = p * q;
    let mut phi_max = (p - 1) * (q - 1);

    while ((p == q) || (gcd(phi_max, e as i64) != 1)) {
        p = gen_prime();
        q = gen_prime();
        max = p * q;
        phi_max = (p - 1) * (q - 1);
    }

    let mut d = ext_euclid(phi_max, e as i64);
    while d < 0 {
        d += phi_max;
    }

    let pub_key = PubKey {modulus: max, exponent: e};
    let pri_key = PriKey {modulus: max, exponent: d};
    (pub_key, pri_key)
}

pub fn mod_mult(a: i64, b: i64, modulus: i64) -> i64 {
    if a == 0 {
        return 0;
    }

    let mut product = a * b;
    if product / a == b {
        return product % modulus;
    }
    if a & 1 != 0 {
        product = mod_mult((a >> 1), b, modulus);
        if(product << 1) > product {
            return ((product << 1) % modulus + b) % modulus;
        }
    }

    product = mod_mult((a >> 1), b, modulus);
    if(product << 1) > product {
        return (product << 1) % modulus;
    }

    let mut sum = 0;
    let mut a = a;
    let mut b = b;
    while b > 0 {
        if b & 1 != 0 {
            sum = (sum + a) % modulus;
        }
        a = (2 * a) % modulus;
        b >>= 1;
    }
    sum
}

pub fn rsa_mod_exp(b: i64, e: i64, m: i64) -> i64 {
    let mut p = 1;
    if b < 0 || e < 0 || m <= 0 {
        return -1;
    }
    let mut b = b % m;
    let mut e = e;
    while e > 0 {
        if e & 1 != 0 {
            p = mod_mult(p, b, m);
        }
        b = mod_mult(b, b, m);
        e >>= 1;
    }
    p
}

pub fn rsa_encrypt(msg: &str, publi: &PubKey) -> Vec<i64> {
    let mut encrypted = Vec::with_capacity(msg.len());
    for c in msg.chars() {
        let encrypted_char = rsa_mod_exp(c as i64, publi.exponent, publi.modulus);
        if encrypted_char == -1 {
            panic!("Encryption failed");
        }
        encrypted.push(encrypted_char);
    }
    encrypted
}

pub fn rsa_decrypt(msg: &[i64], prive: &PriKey) -> String {
    let mut decrypted = String::new();
    for &encrypted_char in msg {
        let decrypted_char = rsa_mod_exp(encrypted_char, prive.exponent, prive.modulus);
        if decrypted_char == -1 {
            panic!("Decryption failed");
        }
        decrypted.push(decrypted_char as u8 as char);
    }
    decrypted
}


fn main() {
    println!("random prime : {}", gen_prime());
    println!("gcd : {}", gcd(56, 98));

    let (pub_key, pri_key) = gen_keys();
    let msg = "Test !";

    let encrypted = rsa_encrypt(msg, &pub_key);
    println!("Encrypted : {:?}", encrypted);

    let decrypted = rsa_decrypt(&encrypted, &pri_key);
    println!("Decrypted : {}", decrypted);
}
