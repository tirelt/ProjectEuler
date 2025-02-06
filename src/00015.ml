let rec fact n acc stop =
  if n = stop then acc else fact (n-1) (acc*n) stop
let rec combine n k =
  fact n 1 k / (fact k 1 1) 

let combine_2 n k =
  let rec aux n k acc =
    if k <1. then acc else aux n (k-.1.) (acc*.(n+.1.-.k)/.k) in
  aux n k 1. |> Float.round 

let _ = 
  Printf.printf "%f\n" (combine_2 40. 20.)
