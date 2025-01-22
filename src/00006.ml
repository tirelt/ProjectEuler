let sum_of_squares n =
  n * (n+1) * (2*n+1) / 6
let sum n = 
  n * (n+1) / 2
let res n = 
  let s = sum n in
    s * s - sum_of_squares n
let _ =
  Printf.printf "%d\n" (res 100)