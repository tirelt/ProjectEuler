let rec gcd a b = 
  if b = 0 then a else gcd b (a mod b) 
let lcm a b = 
  (a * b) / (gcd a b) 
let rec lcm_list = function 
  |[] -> 1
  |x :: xs -> lcm x (lcm_list xs) 
let smallest_multiple n = 
  lcm_list (List.init n (fun i -> i + 1)) 

let _ =
  Printf.printf "%d\n" (smallest_multiple 20)