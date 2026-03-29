module TypeCheck where

import           DependentType.Core         (Term)
import           DependentType.Elaboration  (elaborate)
import           DependentType.Eval         (quote)
import           DependentType.Surface      (SurfaceTerm)
import           DependentType.Typing       (emptyCtx, infer)

typecheck :: SurfaceTerm -> Either String Term
typecheck surface =
    let core = elaborate [] surface
    in  fmap (quote 0) (infer emptyCtx core)
