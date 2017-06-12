
const CDF_LENGTH_D3:u8 = 6;
const CDF_D3:[u16;6] = [602, 1521, 1927, 2031, 2046, 2047];

const B:u32=4;
const B_bar:u32=11;
const two_pow_B:u32=16;
const two_pow_B_bar:u32=2048;
const q:u32=32768;

const n:u32=752;
const n_bar:u32=8;
const m_bar:u32=8;




pub fn lwe_sample_n_inverse_12_slice(input:&mut[u32;(n) as usize]){



    for i in 0..(n) as usize {
        let mut sample:u8=0;
        let rnd:u8 = (input[i] as u8) >>1;
        let sign:u8 = (input[i] as u8)&1;
        
        for j in 0..CDF_LENGTH_D3-1 {
            sample+= ((CDF_D3[j as usize] -rnd as u16) as u8)>>7;
        
        }
        //flips sample if sign==1, does nothign if sample==0
        input[i] = (sample*((sign==0) as u8)+(sample^255)*(1-(sign==0) as u8)+sign) as u32;


    }
}



