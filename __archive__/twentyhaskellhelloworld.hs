-- ghc -o hask hask.hs
{-# LANGUAGE FlexibleInstances #-}

class Animal a where
    makeSound :: a -> String

data Cat = Cat { catName :: String } deriving Show

data Dog = Dog { dogName :: String } deriving Show

instance Animal Cat where
    makeSound _ = "Meow!"

instance Animal Dog where
    makeSound _ = "Woof!"

makeEveryoneTalk :: (Functor f, Animal a) => f a -> f String
makeEveryoneTalk container = fmap makeSound container

main :: IO ()
main = do
    let cattyCat = Cat { catName = "Catty Cat" }
    let doggyDog = Dog { dogName = "Doggy Dog" }
    putStrLn $ (catName cattyCat) ++ " [cat things] " ++ (makeSound cattyCat)
    putStrLn $ (dogName doggyDog) ++ " [dog things] " ++ (makeSound doggyDog)
    let catList = [Cat "Luna", Cat "Milo"]
    let maybeDog = Just (Dog "Buddy")
    let noAnimal = Nothing :: Maybe Cat
    print $ makeEveryoneTalk catList
    print $ makeEveryoneTalk maybeDog
    print $ makeEveryoneTalk noAnimal
