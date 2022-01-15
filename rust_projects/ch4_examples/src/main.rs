fn main() {
  let mut saturday = String::from("dear diary");

  get_ready_for(&mut saturday);

  println!("{}", saturday);
}

fn get_ready_for(some_string: &mut String) {
  some_string.push_str(", i'm so excited to film this weekend. love, rosy");
}


//  can you spot the easter egg in the code? (scroll down for the explanation)












// look at line 4 in the code. `&mut` is actually a real reserved keyword built into Rust language itself. it's a mutable reference, hence the `mut` and the reference is the `&`. a reference just refers to a variable, holding some value like the words "dear diary" or the number "9". mutable means you can mutate or change the variable in this case, even though you're just using it/referring to it, even if you don't own it.

// it's like if you borrowed somebody's submissive for a scene at a play party (references are borrowing, not owning) and you had permission to leave marks on their skin with your implements (mutating with your function)

// but tonight I realized, together, `&mut` looks like smut :) and this is a real little program that runs! lol so in other words, i turned line 4 into a line that reads like english and it still runs :)


