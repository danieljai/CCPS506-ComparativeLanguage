module Self where
    compareWithHundred :: (Num a , Ord a ) => a -> Ordering
    compareWithHundred = compare 100

    applyT :: (a->a) -> a -> a
    applyT f x = f ( f(f x))

    t x = x + 2