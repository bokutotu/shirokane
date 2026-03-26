module Core where

data Term
    = Type
    | Var Int
    | Pi String Term Term
    | Lam String Term Term
    | App Term Term
    deriving (Show, Eq)
