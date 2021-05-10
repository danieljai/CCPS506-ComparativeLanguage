-- Andy Lee
-- 500163559

module Lab3 where
    lab3map p [] = []
    lab3map p (xh:xt) = 
        p xh : lab3map p xt
    

    nth x [] = error "Empty list"
    nth x (yh:yt) =
        if x <=1 || yt == [] then yh
        else nth (x-1) yt


    data Polygon = Polygon Int Float 
        -- deriving (Show)
    
    instance Show Polygon where
        show (Polygon n length) = 
            (show n) ++ "-side polygon at " ++ show(length) ++ " lengths."
    
    instance Eq Polygon where
        (Polygon n1 length1) == (Polygon n2 length2) = (n1 == n2) && (length1 == length2)

    area (Polygon n length) =
        let apothem = length / (2 * tan (pi / fromIntegral(n) ))
        in apothem * length * 0.5 * fromIntegral(n)
    
    --samearea :: Polygon -> Polygon -> Bool
    samearea a b =
        area a == area b