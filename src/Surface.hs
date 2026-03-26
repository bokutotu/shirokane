module Surface where

data SurfaceTerm
    = SType
    | SVar String
    | SLam String SurfaceTerm SurfaceTerm
    | SApp SurfaceTerm SurfaceTerm
    deriving (Show, Eq)
