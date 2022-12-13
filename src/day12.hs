import Data.Char (isUpper, ord)
import Data.List (elem, elemIndex, find, minimumBy, sortBy)
import Data.Map.Strict (Map, empty, findWithDefault, fromList, insert, keys, (!))
import Data.Maybe (fromJust, isNothing)
import Debug.Trace (trace)

type Field = Map Point Char

type Point = (Int, Int)

type Path = [Point]

parseField :: Int -> Int -> [[Char]] -> Field
parseField fieldWidth fieldHeight input =
  foldl parseRow empty [0 .. fieldHeight - 1]
  where
    parseRow field y = foldl (\field x -> insert (x, y) ((input !! y) !! x) field) field [0 .. fieldWidth - 1]

makeStep :: Int -> Int -> Field -> [Path] -> [Path]
makeStep fieldWidth fieldHeight field = concatMap (nextStepForPath fieldWidth fieldHeight field)

isValidPoint :: Int -> Int -> Field -> Path -> (Int, Int) -> Bool
isValidPoint fieldWidth fieldHeight field path candidate@(x, y) =
  (y >= 0) && (y < fieldHeight)
    && (x >= 0)
    && (x < fieldWidth)
    && candidate `notElem` path
    && (ord height - ord currentHeight <= 1)
  where
    (currentX, currentY) = last path
    height = adjustStart (field ! (x, y))
    adjustStart value = if value == 'E' then 'z' else value

    currentHeight = adjustEnd (field ! (currentX, currentY))
    adjustEnd value = if value == 'S' then 'a' else value

nextStepForPath :: Int -> Int -> Field -> Path -> [Path]
nextStepForPath fieldWidth fieldHeight field path =
  map (\point -> path ++ [point]) validNextPoints
  where
    validNextPoints = filter (isValidPoint fieldWidth fieldHeight field path) $ foldl (\acc (dx, dy) -> (currentX + dx, currentY + dy) : acc) [] deltas
    currentPosition@(currentX, currentY) = last path
    deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)]

findShortestPath :: Int -> Int -> Field -> [Path] -> [(Int, Int)]
findShortestPath fieldWidth fieldHeight field startPaths =
  trace (show $ "length of a new path" ++ (show newPaths)) $
    case solutionCandidate of
      Just path -> path
      _ -> findShortestPath fieldWidth fieldHeight field newPaths
  where
    solutionCandidate = find (isFinishPosition field) newPaths
    newPaths = makeStep fieldWidth fieldHeight field startPaths

isFinishPosition :: Field -> [(Int, Int)] -> Bool
isFinishPosition field path = (field ! (x, y)) == 'E' where (x, y) = last path

-- type Distances = Map Point Int

findPosition :: [[Char]] -> Char -> (Int, Int)
findPosition field value =
  (x, y)
  where
    startLine = fromJust . find (\line -> value `elem` line) $ field
    x = fromJust . elemIndex value $ startLine
    y = fromJust . elemIndex startLine $ field

main = do
  content <- readFile "src/inputs/day12"
  let input = lines content

  -- print $ field

  -- print $ isValidPoint field [(4,2)] (5, 2)

  let paths = [[findPosition input 'S']]
  let fieldWidth = length (head input)
  let fieldHeight = length input
  let field = parseField fieldWidth fieldHeight input

  -- print  field
  -- print fieldWidth
  -- print fieldHeight
  -- print $ nextStepForPath fieldWidth fieldHeight field ([(0,0),(0,1),(1,1),(1,2),(2,2),(2,3)])
  -- print $ field!(last [(0,0),(0,1),(1,1)])
  -- let zoo = makeStep fieldWidth fieldHeight field
  -- print $ zoo paths
  -- print $ zoo $ zoo $ zoo $ zoo $ zoo $ zoo $ zoo $ zoo $ zoo $ zoo $ zoo paths

  print $ pred $ length $ findShortestPath fieldWidth fieldHeight field paths

-- let start = findPosition field 'S'
-- let end = findPosition field 'E'
-- print $ getDistance end $ go field (fromList [(start, 0)] []