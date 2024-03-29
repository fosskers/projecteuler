module Euler where

-- Any operator ending with `:` is one of my personal Coli(o)n operators.

(|:) :: Integral a => a -> a -> Bool
a |: b = a `mod` b == 0

(/|:) :: Integral a => a -> a -> Bool
a /|: b = not $ a |: b

primes :: Integral a => [a]
primes = 2 : filter isPrime [3,5..]

isPrime :: Integral a => a -> Bool
isPrime n = n > 1 && all (n /|:) (takeWhile (<= sqrt' n) primes)
    where sqrt' n = floor . sqrt $ fromIntegral n

primeFactors :: Integral a => a -> [a]
primeFactors n = f n primes
    where f n (p:ps) | n < 2     = []
                     | n |: p    = p : f (n `div` p) (p:ps)
                     | otherwise = f n ps
