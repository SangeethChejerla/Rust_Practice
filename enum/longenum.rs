enum MyLongEnumName{
  VarA,
  VarB,
  VarC,
}

fn process_enum(e:MyLongEnumName){
  use MyLongEnumName as E;
  
  match e {
    E::VarA => a(),
    E::VarB=> b(),
    E::VarC=> c(),
  }
}

fn a(){
  println!("Function a was called");
}
fn b(){
  println!("Function b was called");
}
fn c(){
  println!("Function c was called")
}

fn main(){
  let enum_var= MyLongEnumName::VarB;
  process_enum(enum_var)
}
