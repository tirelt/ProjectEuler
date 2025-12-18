let rec is_prime_naive n i =
  if i = 1 then true
  else if n mod i = 0 then false
  else is_prime_naive n (i-1)

let rec find_prime n acc =
  if n = 1 then acc
  else if is_prime_naive n (n-1) then find_prime (n-1) (n::acc)
  else find_prime (n-1) acc

let rec find_power n p acc =  
  if n mod p = 0 then find_power (n/p) p (acc+1)
  else acc

let rec find_factors array_factor primes n = 
  match primes with
  |[] -> ()
  |h::q -> let power = find_power n h 0 in array_factor.(h)<-max power array_factor.(h); find_factors array_factor q n

let rec pow n i acc = if i=0 then acc else pow n (i-1) (acc*n)

let rec construct_from_factors i acc = function
  |[] -> acc
  |h::q -> if h=0 then construct_from_factors (i+1) acc q else construct_from_factors (i+1) (acc*pow i h 1) q

let compute_prod_prime n =
  let primes = find_prime n [] in
    let array_factor = Array.make n 0 in
      let rec aux n = if n=0 then () else (find_factors array_factor primes n;aux (n-1)) in
        aux n;
  construct_from_factors 0 1 (Array.to_list array_factor)

let _ =
  Printf.printf "%d\n" (compute_prod_prime 20)