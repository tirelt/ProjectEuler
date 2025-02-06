let rec multiply fact carry new_lst = function
  [] -> if carry = 0 then List.rev new_lst else List.rev (carry::new_lst)
  |h::q -> let temp = h*fact+carry in multiply fact (temp/10) ((temp mod 10)::new_lst) q

let ret = 
  let rec iterate n lst = 
    if n = 1000 then lst else (iterate (n+1) (multiply 2 0 [] lst)) in
  iterate 0 [1] |> List.fold_left (+) 0 