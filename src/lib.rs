extern crate wasm_bindgen;
#[macro_use]
extern crate lazy_static;
use wasm_bindgen::prelude::*;
use noise::{NoiseFn, Perlin, Billow, Checkerboard, Cylinders, Worley, Fbm};






lazy_static! {
    static ref PERLIN : Perlin = Perlin::new();
    static ref BILLOW : Billow = Billow::new();
    static ref CHECKERBOARD : Checkerboard = Checkerboard::new();
    static ref CYLINDERS : Cylinders = Cylinders::new();
    static ref WORLEY : Worley = Worley::new();
    static ref FBM : Fbm = Fbm::new();
}


#[wasm_bindgen]
pub fn render(width: i32, height: i32, t: i32) -> String {
    let mut lines = String::from("");
    let mut wordI = 0;
    let WORDS = String::from("Far far away, behind the word mountains, far from the countries Vokalia and Consonantia, there live the blind texts. Separated they live in Bookmarksgrove right at the coast of the Semantics,a large language ocean. A small river named Duden flows by their place and supplies it with the necessary regelialia. It is a paradisematic country, in which roasted parts of sentences fly into your mouth. Even the all-powerful Pointing has no control about the blind texts it is an almost unorthographic life One day however a small line of blind text by the name of Lorem Ipsum decided to leave for the far World of Grammar. The Big Oxmox advised her not to do so, because there were thousands of bad Commas, wild Question Marks and devious Semikoli, but the Little Blind Text didn’t listen. She packed her seven versalia, put her initial into the belt and made herself on the way."); 

    for x in 0..height / 9 {
        let mut line = String::from("");
        for y in 0..width / 8 {
            // options
            let n = WORLEY.get([(x as f64) / 20.0 + 0.1, (y as f64) / 20.0 + 0.1, t as f64 / 2000.0]);
            
            // noise
            // if n > 0.35 {
            //     // line.push_str("&nbsp;");
            //     line.push('*');
            // }
            // else if n > 0.25 {
            //     line.push('.');
            // }
            // else if n > 0.15 {
            //     line.push('~');
            // }
            // else if n > 0.0 {
            //     line.push('H');
            // }
            if n > 0.35 {
                line.push_str("&nbsp;");
            }
            else if n > 0.25 {
                line.push_str(".");
            }
            else if n > 0.15 {
                line.push_str("~");
            }
            else if n > 0.0 {
                line.push_str("H3");
            }
            else if n > -0.2 {
                wordI += 1;
                if wordI == WORDS.len() - 5 {
                    wordI = 0;
                }

                let c: char = WORDS.chars().nth(wordI).unwrap();
                
                if c == ' ' {
                    line.push_str("&nbsp;");
                } else {
                    line.push(c);
                }
            } 
            else {
                line.push_str("&blk34;");
            }            
        }

        lines.push_str(line.as_str());
        lines.push_str("\n");
    }

    return lines
}




#[wasm_bindgen]
pub fn real_text(width: i32, height: i32, t: i32) -> String {
    let mut lines = String::from("");

    for x in 0..height / 9 {
        let mut line = String::from("");
        for y in 0..width / 8 {
            // options
            // let n = PERLIN.get([(x as f64) / 4.0 + (t as f64) / 200.0 + 0.1, (y as f64) / 4.0 + 0.1]);
            // let n = PERLIN.get([(x as f64) / 20.0 + 0.1, (y as f64) / 20.0 + 0.1, t as f64 / 2000.0]);
            // let n = CYLINDERS.get([(x as f64) / 20.0 + 0.1, t as f64 / 1500.0]);
            // let n = CYLINDERS.get([(x as f64 + t as f64 / 200.0) / 20.0 + 0.1, (y as f64 + t as f64 / 200.0) / 20.0]);
            // let n = CHECKERBOARD.get([(x as f64) / 4.0 + (t as f64) / 200.0 + 0.1, (y as f64) / 4.0 + 0.1]);
            // let n = WORLEY.get([(x as f64 + t as f64 / 200.0) / 20.0 + 0.1, (y as f64 + t as f64 / 200.0) / 20.0]);
            let n = WORLEY.get([(x as f64) / 20.0 + 0.1, (y as f64) / 20.0 + 0.1, t as f64 / 2000.0]);
            // let n =  FBM.get([(x as f64) / 20.0 + 0.1, (y as f64) / 20.0 + 0.1, t as f64 / 2000.0]);

            // noise
            if n > 0.35 {
                line.push_str("&nbsp;");
            }
            else if n > 0.25 {
                line.push_str(".");
            }
            else if n > 0.15 {
                line.push_str("~");
            }
            else if n > 0.0 {
                line.push_str("H3");
            }
            else if n > -0.2 {
                line.push_str("#");
            } 
            else {
                line.push_str("&blk34;");
            }            
        }

        lines.push_str(line.as_str());
        lines.push_str("\n");
    }

    return lines
}
