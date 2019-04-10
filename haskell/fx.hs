doubleMe x = x * 2
sumList x = x !! 0 + x !! 1

head' :: [a] -> a
head' [] = error "DONT MESS WITH ME"
head' (x:_) = x

length' :: [a] -> Int
length' [] = 0
length' (_:x) = 1 + length' x

sum' :: (Num a) => [a] -> a
sum' [] = 0
sum' (xs:x) = xs + sum' x

capital :: String -> String
capital "" = "Empty string, whoops!"
capital all@(x:xs) = "The first letter of " ++ all ++ " is " ++ [x]

getfirst :: Int -> Int -> Int
a `getfirst`  b = a

whereami :: Int -> [Char]
whereami a
    | mult < 10     = "ax2 is < 10"
    | mult - 5 < 10 = "ax2 - 5 < 10"
    where mult = a * 2

normal_guard :: Int -> Int
normal_guard a = a + mult + b
    where mult = 10
          b = 5

initials :: String -> String
initials firstname = [f]
    where (f:_) = firstname

cylinder :: (RealFloat a) => a -> a -> a
cylinder r h = sideArea + 2 * topArea
    where sideArea = 2 * r * h
          topArea = r ^ 2


letcylinder :: (RealFloat a) => a -> a -> a
letcylinder r h =
    let sideArea = 2 * r * h
        topArea = r ^ 2
    in  sideArea + 2 * topArea

fibs = 0 : 1 : [ a + b | (a, b) <- zip fibs (tail fibs)]

binumber :: Int -> [Char]
binumber a = case a of 10 -> "NUMBER IS 10"
                       b  -> "GENERAL NUMBER " ++ show b

number :: Int -> [Char]
number 10 = "NUMBER IS 10"
number a  = "GENERAL NUMBER " ++ show a

factorial :: Int -> Int
factorial 0 = 1
factorial n = n * factorial (n - 1)

factorial' :: Int -> Int
factorial' x
    | basecase  = 1
    | otherwise = x * factorial' (x - 1)
    where basecase = x <= 1

factorial'' :: Int -> Int
factorial'' m = case m of 0 -> 1
                          m -> m * factorial'' (m - 1)

factorial''' :: Int -> Int
factorial''' t = input t
    where input 0 = 1
          input t = t * factorial''' (t - 1)

factorial'''' :: Integer -> Integer
factorial'''' z = product [1..z]

maximum' :: (Ord l) => [l] -> l
maximum' [x] = x
maximum' a = max (head a) (maximum' (tail a))

take_singular_list :: [Int] -> Int
take_singular_list [a] = a
