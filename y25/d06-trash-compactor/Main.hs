import Data.List (transpose)

part1 :: String -> Integer
part1 input = sum $ zipWith selectValue (words $ last inputLines)
                                       (map (\col -> (sum col, product col))
                                            (transpose $ map (map read . words)
                                                            (init inputLines)))
  where
    inputLines = lines input
    selectValue "+" (s, _) = s
    selectValue "*" (_, p) = p
    selectValue _ _ = error "Invalid operator"

main :: IO ()
main = do
    input <- readFile "../../inputs/2025-06.txt"
    putStrLn $ "Part 1: " ++ show (part1 input)
