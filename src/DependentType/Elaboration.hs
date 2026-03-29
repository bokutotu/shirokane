module DependentType.Elaboration where

import           DependentType.Core    (Term (..))
import           DependentType.Surface (SurfaceTerm (..))
import           Prelude               hiding (lookup)

elaborate :: [String] -> SurfaceTerm -> Term
elaborate _ SType = Type
elaborate ctx (SVar name) = case lookup name ctx of
    Just n  -> Var n
    Nothing -> error $ "Unbound variable: " ++ name
elaborate ctx (SLam name ty body) = Lam name (elaborate ctx ty) (elaborate (name : ctx) body)
elaborate ctx (SApp f x) = App (elaborate ctx f) (elaborate ctx x)
elaborate ctx (SAnn t ty) = Ann (elaborate ctx t) (elaborate ctx ty)

lookup :: String -> [String] -> Maybe Int
lookup name = go 0
  where
    go _ [] = Nothing
    go n (y : ys)
        | name == y = Just n
        | otherwise = go (n + 1) ys
