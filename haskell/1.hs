#!/usr/bin/runhaskell
{-# LANGUAGE OverloadedStrings #-}

import Data.Set (Set)
import qualified Data.Set as Set
import qualified Data.Text as T
import qualified Data.Text.IO as T

solve :: [Int] -> [Int]
solve = scanl1 (+)

solve2 :: [Int] -> T.Text
solve2 = T.pack . show . first_repeating Set.empty . solve . cycle
  where
    first_repeating set (x:xs) = if x `Set.member` set then x
                                 else first_repeating (Set.insert x set) xs

main :: IO ()
main = do
  input <- T.readFile "../1_input_1"
  let input' = map (read . T.unpack . T.dropWhile (== '+')) . filter (/= "") . T.lines $ input
  T.putStrLn $ solve1 input' <> "\n" <> solve2 input'
    where
      solve1 = T.pack . show . last . solve

