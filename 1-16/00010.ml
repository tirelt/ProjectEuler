let prime_array primes =
  let n = Array.length primes in
    primes.(0) <- false;
    primes.(1) <- false;
    for i=2 to (n-1) do 
      if(primes.(i)) then 
        let counter = ref 2 in
        while !counter*i<n do
          primes.(!counter*i) <- false;
          counter := !counter + 1;
        done
    done 

let rec sum_aux primes i acc =
  if i =  Array.length primes then acc else sum_aux primes (i+1) (acc+if primes.(i) then i else 0)

let sum_primes n = 
  let primes = (Array.make (n+1) true) in 
    primes |> prime_array;
    sum_aux primes 0 0 

let _ =   
  Printf.printf "%d\n" (sum_primes 2000000)