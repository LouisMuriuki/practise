pub fn roman_numbers(roman_number:&str) ->i32{
let mut sum=0;
let vec_roman:Vec<char>=roman_number.chars().collect();

for i in 0..vec_roman.len(){
  if vec_roman[i]=='I'{
    sum+=1;
  }else if vec_roman[i]=='V'{
    sum+=5;
  } else if vec_roman[i]=='X'{
    sum+=10;
  } else if vec_roman[i]=='L'{
    sum+=50;
  } else if vec_roman[i]=='C'{
    sum+=100;
  } else if vec_roman[i]=='D'{
    sum+=500;
  } else if vec_roman[i]=='M'{
    sum+=1000;
  }
}
sum


}
