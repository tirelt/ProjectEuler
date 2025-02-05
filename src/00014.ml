
let rec aux k c = 
  if k = 1 then c else 
    if k mod 2 = 0 then aux (k/2) (c+1) else aux (3*k+1) (c+1)

let rec test n max_ite n_max = 
  if n = 1000000 then n_max else begin 
    let ite = aux n 1 in
    if ite > max_ite then test (n+1) ite n else  test (n+1) max_ite n_max 
  end

let a = test 1 0 0