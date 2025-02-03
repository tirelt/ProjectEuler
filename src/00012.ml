let count_divisors n = 
  let rec aux d count =
    if d*d>n then count 
    else if d*d=n then count+1
    else if n mod d = 0 then aux (d+1) (count+2)
    else aux (d+1) count
  in
    aux 1 0

let find div_count_target =
  let rec aux n =
    let tri_num =n*(n+1)/2 in
    if count_divisors tri_num >= div_count_target then tri_num
    else aux (n+1)
  in 
  aux 1000

let triangle n = 
  n*(n+1)/2

let _ =   
  Printf.printf "%d\n" (find 500)