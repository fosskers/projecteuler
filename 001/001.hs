{-
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
-}

(..|..) :: Integral a => a -> a -> Bool
a ..|.. b = a `mod` b == 0

f :: Int
f = sum $ filter (\n -> n ..|.. 3 || n ..|.. 5) [1..999]
