//the fim functions assume that the size of the matrix is a multiply of 4 in order to reduce number of aes calls.

#![feature(asm,concat_idents,const_fn)]
#![no_std]
#![feature(type_ascription)]

//extern crate kernel;

extern crate sam4l;
use sam4l::aesa;

//extern fn lwe_sample_n_inverse_12(in:&mut[u32;64]);
mod d_3;

use d_3::lwe_sample_n_inverse_12_slice;


const B:u16=4;
const B_bar:u16=11;
const two_pow_B:u16=16;
const two_pow_B_bar:u16=2048;
const q:u16=32768;

const n:u16=752;
const m:u16=8;
const n_bar:u16=8;
const m_bar:u16=8;

pub fn round (input:u16) -> u16{

    return (((input+1024)%32768)>>B_bar);
}

pub fn cross (input:u16) -> u16{

    return ((input%32768)/4096)%2;
}


pub fn greater_than (a:u16, b:u16) -> u16{
    let mut changed:u16 = 0;
    let mut output:u16 = 0;
    
    let pow2: [u16; 16] = [32768, 16384, 8192, 4096, 2048, 1024, 512,  256, 128, 64, 32, 16, 8, 4, 2, 1];
    
    for i in 0..16{
        let bita=(a&pow2[i])>>(15-i);
        let bitb=(b&pow2[i])>>(15-i);
        output = (changed ^ 0)*output+ (changed^1)*bita*(bita ^ bitb);
		changed = (changed ^ 0) * changed + (changed ^ 1) * ( changed ^ (bita ^ bitb));
    }
    
    
    return output;
    
}

pub fn rec (w:u16, b:u16) -> u16{
    let equal:u16 = (cross(w) & b) + ((1^cross(w)) & (b^1));
    let outputa:u16=equal*w;
    
    let want_high:u16=greater_than(w%4096,2047);
    let outputb:u16=(want_high ^ 1)*(  ((w-w%4096)-1)%32768 )+(want_high ^ 0)*(  ((w-w%4096+4096))%32768 );
    let output:u16 = outputa+(equal ^1)*outputb;
    
    return round(output);

}


pub fn alice_part1(mut b: &'static mut[[u16; 8]; 752] ){

    //pub static mut A: &'static mut[[u16; 752]; 752] = &mut [[0; 752]; 752];
    pub static mut A_slice: &'static mut[u16; 752] = &mut [0; 752];
    
    pub static mut s: &'static mut[[u16; 8]; 752] = &mut [[0; 8]; 752];
    pub static mut e: &'static mut[[u16; 8]; 752] = &mut [[0; 8]; 752];
    
    //pub static mut b: &'static mut[[u16; 8]; 752] = &mut [[0; 8]; 752];
    
    //This is what Alice does
    
    unsafe{
    
        //for i in 0..n {
        //    for j in 0..n {
        //        A[i as usize][j as usize]=i+j;
        //    }
        //}
        
        for i in 0..m {
            for j in 0..m {
                s[i as usize][j as usize]=1;
                
            }
        }
        

        
        for i in 0..n {
            for i1 in 0..n {
                    //col
                    A_slice[i1 as usize] = i+i1;
                }
            for j in 0..m {
                
                let mut temp:u16=e[i as usize][j as usize];
                for i1 in 0..n {
                    temp+=A_slice[i1 as usize]*s[i1 as usize][j as usize];
                    //temp+=A[i as usize][i1 as usize]*s[i1 as usize][j as usize];
                    //b[i as usize][j as usize]+A[i as usize][i1 as usize]*s[i1 as usize][j as usize];
                }
                b[i as usize][j as usize]=temp;
            }
        }
        
    
    }
}


pub fn bob_part(mut bp: &'static mut[[u16; 752]; 8] ,mut c: &'static mut[[u16; 8]; 8] ,mut k_bob: &'static mut[[u16; 8]; 8],mut b: &'static mut[[u16; 8]; 752] ){
    //This is what Bob does
    
    pub static mut A_slice: &'static mut[u16; 752] = &mut [0; 752]; 
    
    pub static mut sp: &'static mut[[u16; 752]; 8] = &mut [[0; 752]; 8];
    pub static mut ep: &'static mut[[u16; 752]; 8] = &mut [[0; 752]; 8];
    
    pub static mut bp: &'static mut[[u16; 752]; 8] = &mut [[0; 752]; 8];
    
    
    unsafe{
    

        
        for i in 0..m {
            for j in 0..m {
                sp[i as usize][j as usize]=1;

            }
        }
        
        for i in 0..m {
            for j in 0..n {
                for i1 in 0..n{
                    //row
                    A_slice[i1 as usize]=i1+j;
                }
                let mut temp:u16=ep[i as usize][j as usize];
                for i1 in 0..n {
                    temp+=bp[i as usize][j as usize]+sp[i as usize][i1 as usize]*A_slice[i1 as usize];
                }
                bp[i as usize][j as usize]=temp;
            }
        }
        
    
    }
    
    pub static mut epp: &'static mut[[u16; 8]; 8] = &mut [[0; 8]; 8];
    
    pub static mut v: &'static mut[[u16; 8]; 8] = &mut [[0; 8]; 8];
    
    unsafe{
    
        for i in 0..m {
            for j in 0..m {
                let mut temp:u16=0;
                for i1 in 0..n {
                    temp+=v[i as usize][j as usize]+sp[i as usize][i1 as usize]*b[i1 as usize][j as usize];
                }
                v[i as usize][j as usize]=temp;
                
                c[i as usize][j as usize]=cross(v[i as usize][j as usize]%q);
            }
        }
        
        for i in 0..m {
            for j in 0..m {                
                k_bob[i as usize][j as usize]=round(v[i as usize][j as usize]%q);
            }
        }
        
    }


}

pub fn alice_part2(mut bp: &'static mut[[u16; 752]; 8]  ,mut c: &'static mut[[u16; 8]; 8] ,mut k_alice: &'static mut[[u16; 8]; 8] ){
    pub static mut s: &'static mut[[u16; 8]; 752] = &mut [[0; 8]; 752];
    pub static mut bp_s: &'static mut[[u16; 8]; 8] = &mut [[0; 8]; 8];
    unsafe{
        for i in 0..m {
            for j in 0..m {
                s[i as usize][j as usize]=1;
                
            }
        }
        for i in 0..m {
            for j in 0..m {
                let mut temp:u16=0;
                for i1 in 0..n {
                    temp+=bp[i as usize][i1 as usize]*s[i1 as usize][j as usize];
                }
                bp_s[i as usize][j as usize]=temp;
            }
        }
        
        for i in 0..m {
            for j in 0..m {
                let mut temp:u16=bp_s[i as usize][j as usize]%q;
                k_alice[i as usize][j as usize]=rec(temp,c[i as usize][j as usize]);
            }
        }
    }

}












