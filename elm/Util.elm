module Util where


zip : List a -> List b -> List (a,b)
zip listX listY =
  case (listX, listY) of
    (x::xs, y::ys) -> (x,y) :: zip xs ys
    (  _  ,   _  ) -> []

zip3 : List a -> List b -> List c -> List (a,b,c)
zip3 listX listY listZ =
  case (listX, listY, listZ) of
    (x::xs, y::ys, z::zs) -> (x,y,z) :: zip3 xs ys zs
    (  _  ,   _  ,   _  ) -> []
