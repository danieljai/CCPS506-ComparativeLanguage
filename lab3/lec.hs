module Test where
  cmp2 :: Ord  a => a -> a -> [Char]
  cmp2 x y | x <y ="first is smaller"
           | x > y = "second is smaller"
           | otherwise = "equal"
  
  cmp3 x y | x+1 < y+1 = "first one is smaller"
           | x+1 > y+1 = "second one is smaller"
           | otherwise = "equal"

  data Pt = Pt3 Float Float Float | Pt2 Float Float
            --deriving (Show)

  ptX (Pt2 x _) = x
  ptX (Pt3 x _ _) = x

  ptY (Pt2 _ y ) = y
  ptY (Pt3 _ y _) = y

  ptz (Pt3 _ _ z) = z

  vecLen (Pt2 x y) = sqrt(x^2+y^2)
  vecLen (Pt3 x y z) = sqrt(x^2+y^2+z^2)

  ptAdd (Pt2 x1 y1) (Pt2 x2 y2) = Pt2 (x1+x2) (y1+y2)

  instance Eq Pt where
    (Pt2 x1 y1) == (Pt2 x2 y2) = (x1==x2 && y1==y2)
  
  instance Num Pt where
    (Pt2 x1 y1) + (Pt2 x2 y2) = Pt2 (x1+x2) (y1+y2)
    (Pt2 x1 y1) - (Pt2 x2 y2) = Pt2 (x1-x2) (y1-y2)
    (Pt2 x1 y1) * (Pt2 x2 y2) = Pt2 (x1*x2) (y1*y2)
    abs (Pt2 x1 y1) = Pt2 (abs x1) (abs y1)
    signum (Pt2 x1 y1) = Pt2 (signum x1) (signum y1)
    fromInteger n = let a = (fromInteger n) in Pt2 a a

  instance Show Pt where
    show (Pt2 x y) = 
      "< " ++ (show x) ++ ", " ++ (show y) ++ " >"
  
  chkAxis :: (Float, Float) -> (Float, Float)
  chkAxis (0,_) = (0,1)
  chkAxis (_,0) = (1,0)
  chkAxis (a,b) = (a,b)

  chkClr rgb =
    case rgb of 
      (255,_,_) -> "Red"
      (_,255,_) -> "Green"
      (_,_,255) -> "Blue"
      -- (_,_,_) -> "None"
      x -> "None"

  pos x = x >= 0

  filt p [] = []
  filt p (xh:xt) =
    if p xh then xh : filt p xt
    else filt p xt
  
  roots a b c =
    let disc = sqrt(b*b - 4*a*c)
    in ((-b + disc)/(2*a),(-b - disc)/(2*a))
  
  andy a b c =
    let thor = b*b
    in (thor +a+c)
