-- igoroffline :: ghc -o hask hask.hs
data Animal = Animal 
  { animalId  :: Int
  , animalAge :: Int 
  } deriving (Show)

main :: IO ()
main = do
    let firstAnimal = Animal { animalId = 1, animalAge = 18 }
    let secondAnimal = Animal { animalId = 2, animalAge = 19 }
    print firstAnimal
    print secondAnimal
