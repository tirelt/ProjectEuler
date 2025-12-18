let test n =
  if n mod 3 = 0 || n mod 5 = 0 then n else 0

let rec func n acc = 
  if n = 0 then acc else func (n-1) (acc + test n)

let _ =   
  Printf.printf "%d\n" (func 999 0)