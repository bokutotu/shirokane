module DependentType.Surface where

data SurfaceTerm
    = SType
    | SVar String
    | SLam String SurfaceTerm SurfaceTerm
    | SApp SurfaceTerm SurfaceTerm
    | SAnn SurfaceTerm SurfaceTerm
    deriving (Show, Eq)
