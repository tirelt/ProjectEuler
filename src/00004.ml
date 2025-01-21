let is_palindromic n =
  let n_str = string_of_int n |> String.to_seq |> List.of_seq in
    n_str |> List.rev |> (fun l -> l = n_str)


let rec find_largest n m largest = 
  let new_largest = let prod = n*m in if is_palindromic prod && prod>largest then prod else largest in
  match n,m with
  999,999 -> new_largest
  |999,_ ->  find_largest (m+1) (m+1) new_largest
  |_,_ -> find_largest (n+1) m new_largest

let _ =
  Printf.printf "%d\n" (find_largest 100 100 0)