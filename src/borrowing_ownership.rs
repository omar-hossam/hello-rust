fn main() {
    
    let mut score = String::from("Level 1");
    
    let ref1 = &mut score; // this variable changed the value of another variable??
    // so this variable references to the value of the variable
    
    ref1.push_str(" - Started");
    ref1.push_str(" - Almost done");
    
    let ref2 = &score;
    
    println!("Final score: {}", ref2);
    
    /* what is the difference between &str and String::from()?
     * -------------------------------------------------------
     * &str -> references to a string value BUT doesn't own it
     * String -> OWNS a string value
    */
    
    /* What I learned?
     * you can have ONLY ONE mutable reference to the mutable variable
     * this mutable reference can CHANGE the value of the original variable BUT it doesn't take the ownership of the value!
     * we can make multiple immutable references FOR reading/getting the value from a variable
     */
     
     /* Rule 1:
      * -------
      * wanna CHANGE value of a variable? -> use `&mut varname` and make sure varname is mutable itself*
      * wanna READ value of a ~ ? -> use `&varname` 
     */ 
}
