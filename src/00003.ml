let rec find_divisor n divisor = 
  if divisor * divisor > n then n
  else begin
    if n mod divisor = 0 then find_divisor (n / divisor) divisor else find_divisor n (divisor+1)
  end
  
let _ =
  Printf.printf "%d\n" (find_divisor 600851475143 2)