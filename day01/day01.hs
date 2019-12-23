import Control.Monad

fuel :: (Integral a) => a -> a
fuel x = (x `div` 3) - 2 

recursiveFuel :: (Integral a) => a -> a
recursiveFuel x
    | (fuel x) <= 0 = 0
    | otherwise = fuel x + recursiveFuel (fuel x)

problem1 :: (Integral a) => [a] -> a
problem1 = sum . (fmap fuel)
problem2 :: (Integral a) => [a] -> a
problem2 = sum . (fmap recursiveFuel)

main = do
    lines <- lines <$> readFile "problem_input.txt"
    let masses = read <$> lines
    putStrLn $ "Solution 1: " ++ (show $ problem1 masses)
    putStrLn $ "Solution 2: " ++ (show $ problem2 masses)
    