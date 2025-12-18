let is_prime n =
  let rec check_divisor d =
    d*d > n || (n mod d <> 0 && check_divisor (d+1))
  in
    n > 1 && check_divisor 2
let rec find_nth_prime n count candidate =
  if n = count then candidate-1
  else if is_prime candidate then 
    find_nth_prime n (count + 1) (candidate + 1) 
  else find_nth_prime n count (candidate + 1)