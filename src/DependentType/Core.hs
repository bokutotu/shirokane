module DependentType.Core where

data Term
    = Type
    | Var Int
    | Pi String Term Term
    | Lam String Term Term
    | App Term Term
    | Ann Term Term
    | Sigma String Term Term
    | Pair Term Term
    | Fst Term
    | Snd Term
    deriving (Show, Eq)
