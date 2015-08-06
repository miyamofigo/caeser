#![feature(convert)]

#[test]
fn it_works() {

  let cypher = encode("helloworld", 3);
  assert!("helloworld" == decode(cypher.as_str(), 3)); 

}

fn encode(enc: &str, offset: u32) -> String {

  let offset = offset % 26 + 26;
  let encoded: String = 
                enc.chars()
                   .map( |c| { 
                               let res = match c.is_alphabetic() {

                                           true => { 
                                             let base = 
                                               if c.is_uppercase() 
                                                   { 65 } else { 97 };
                                             let d = base + ((c as u32) - base + offset) % 26;
                                             std::char::from_u32(d).unwrap()
                                           },
                                           _ => c 

                                       }; 
                               res  
                          } )
                   .collect();
  encoded
                  
} 

fn decode(enc: &str, offset: u32) -> String {
    
  encode(enc, 26 - offset)

}  
