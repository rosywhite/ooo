fn main() {
  let mut saturday = String::from("dear diary");

  get_ready_for(&mut saturday);

  println!("{}", saturday);

  let my_string = String::from("hello world");

  //let word = first_word(&s); // word will get the value 5
  //s.clear();

  //println!("{}", word);

  // `first_word` works on slices of `String`s, whether partial or whole
  let word = first_word(&my_string[0..6]);
  let word = first_word(&my_string[..]);
  // `first_word` also works on references to `String`s, which are equivalent
  // to whole slices of `String`s
  let word = first_word(&my_string);

  let my_string_literal = "hello world";

  // `first_word` works on slices of string literals, whether partial or whole
  let word = first_word(&my_string_literal[0..6]);
  let word = first_word(&my_string_literal[..]);

  // Because string literals *are* string slices already,
  // this works too, without the slice syntax!
  let word = first_word(my_string_literal);

}

fn first_word(s: &str) -> &str  {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

fn get_ready_for(some_string: &mut String) {
  some_string.push_str(", i'm so excited to film this weekend. love, rosy");
}


//  can you spot the easter egg in the code? (scroll down for the explanation)












// look at line 4 in the code. `&mut` is actually a real reserved keyword built into Rust language itself. it's a mutable reference, hence the `mut` and the reference is the `&`. a reference just refers to a variable, holding some value like the words "dear diary" or the number "9". mutable means you can mutate or change the variable in this case, even though you're just using it/referring to it, even if you don't own it.

// it's like if you borrowed somebody's submissive for a scene at a play party (references are borrowing, not owning) and you had permission to leave marks on their skin with your implements (mutating with your function)

// but tonight I realized, together, `&mut` looks like smut :) and this is a real little program that runs! lol so in other words, i turned line 4 into a line that reads like english and it still runs :)


