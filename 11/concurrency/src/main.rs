#![allow(dead_code)]

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::fmt::Display;


// Arc: reading from vector w/o moving
fn arc1_readonly<T: 'static + Display + Send + Sync>(vector: Vec<T>) {
    let len = vector.len();
    let a = Arc::new(vector);
    for i in 0..len {
        let v = a.clone();
        thread::spawn(move || {
            println!("{}", v[i]);
        });
    }
}

// Arc & Mutex: increment each number in a vector
fn arc2_increment(vector: Vec<i32>) {
    let mut threads = vec![];
    let len = vector.len();

    let mutex = Arc::new(Mutex::new(vector));
    for i in 0..len {
        let mutex = mutex.clone();
        let handler = thread::spawn(move || {
            let mut vec = mutex.lock().unwrap();
            vec[i] += 1;
        });
        threads.push(handler);
    }

    for thread in threads {
        let _ = thread.join();
    }
    println!("{:?}", *mutex.lock().unwrap());
}

// Arc: increment each number in a vector
fn iter1(mut vec: Vec<i32>) {
    let v2 = &vec.iter_mut().map(|i| {
        *i+1
    }).collect::<Vec<i32>>();

    println!("{:?}", v2);
}

extern crate rayon;
use rayon::prelude::*;

// Arc: increment each number in a vector
fn iter2_par(mut vec: Vec<i32>) {
    vec.par_iter_mut().for_each(|i| {
        println!("{}", *i);
        *i = *i * *i
    });
}

fn main() {
    let vec = (0..20).collect();
    //arc1_readonly(vec![1,2,3]);
    //arc2_increment(vec![1,2,3]);
    //iter1(vec);
    iter2_par(vec);
}
