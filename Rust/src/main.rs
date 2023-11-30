/*
    Licensed under the MIT License given below.
    Copyright 2023 Daniel Lidstrom
    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the “Software”), to deal in
    the Software without restriction, including without limitation the rights to
    use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
    the Software, and to permit persons to whom the Software is furnished to do so,
    subject to the following conditions:
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
    COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
    IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
    CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

mod neural;

use neural::*;

fn xor(i: i32, j: i32) -> i32 { i ^ j }
fn xnor(i: i32, j: i32) -> i32 { 1 - xor(i, j) }
fn and(i: i32, j: i32) -> i32 { i & j }
fn nand(i: i32, j: i32) -> i32 { 1 - and(i, j) }
fn or(i: i32, j: i32) -> i32 { i | j }
fn nor(i: i32, j: i32) -> i32 { 1 - or(i, j) }

fn main() {
    let mut r = neural::rnd::Rnd::new();
    let mut trainer = Trainer::new(2, 2, 6, &mut r);

    let mut inputs = Matrix::new();
    let mut outputs = Matrix::new();
    for i in 0..=1 {
        for j in 0..=1 {
            inputs.push(vec![i as f64, j as f64]);
            outputs.push(vec![
                xor(i, j) as f64,
                xnor(i, j) as f64,
                or(i, j) as f64,
                and(i, j) as f64,
                nor(i, j) as f64,
                nand(i, j) as f64]);
        }
    }
    let lr = 1.0;
    const ITERS: usize = 4000;
    for i in 0..ITERS {
        let input = &inputs[i % inputs.len()];
        let output = &outputs[i % outputs.len()];
        trainer.train(&input, &output, lr);
    }

    let network = &trainer.network;
    println!("Result after {ITERS} iterations");
    println!("        XOR   XNOR    OR   AND   NOR   NAND");
        for i in 0..inputs.len() {
        let input = &inputs[i];
        let output = network.predict(input);
        println!(
            "{:.0},{:.0} = {:.3}  {:.3} {:.3} {:.3} {:.3}  {:.3}",
            input[0],
            input[1],
            output[0],
            output[1],
            output[2],
            output[3],
            output[4],
            output[5]);
    }

    println!("network: {:#?}", network);
}
