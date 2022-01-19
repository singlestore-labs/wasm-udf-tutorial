wit_bindgen_rust::export!("split.wit");                                          
struct Split;                                                                    
use crate::split::Subphrase;                                                     
                                                                                 
fn addr_of(s: &str) -> usize {                                                   
    s.as_ptr() as usize                                                          
}                                                                                
                                                                                 
impl split::Split for Split {                                                    
                                                                                 
    fn split_str(phrase: String, delim: String) -> Vec<Subphrase> {              
        phrase                                                                   
            .split(&delim)                                                       
            .map(|s|                                                             
                Subphrase {                                                      
                  str: s.to_string(),                                            
                  idx: (addr_of(s) - addr_of(phrase.as_str())) as i32            
                })                                                               
            .collect()                                                           
    }                                                                            
} 

