#!/usr/bin/runhaskell
{-# LANGUAGE OverloadedStrings #-}

import Data.Set (Set)
import qualified Data.Set as Set
import qualified Data.Text as T
import qualified Data.Text.IO as T

solve :: [Int] -> [Int]
solve = scanl1 (+)

solve2 :: [Int] -> T.Text
solve2 list = T.pack . show . head . map fst . filter (uncurry Set.member) . zip l . scanl (flip Set.insert) Set.empty $ l
  where l = solve . cycle $ list

main :: IO ()
main = do
  input <- T.readFile "../rust/input/2018/day1.txt"
  let input' = map (read . T.unpack . T.dropWhile (== '+')) . filter (/= "") . T.lines $ input
  T.putStrLn $ solve1 input' <> "\n" <> solve2 input'
    where
      solve1 = T.pack . show . last . solve

