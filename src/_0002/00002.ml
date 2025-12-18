let rec fib_sum a b acc =
  (*Printf.printf "%d %d %d\n" a b acc; *)
  if a > 4000000 then acc 
  else 
      if a mod 2 = 0 then fib_sum b (a+b) (acc + a) 
      else fib_sum b (a+b) acc

let _ =   
  Printf.printf "%d\n" (fib_sum 1 2 0)