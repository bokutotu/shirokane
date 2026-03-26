module Surface where

data SurfaceTerm
    = SType
    | SVar String SurfaceTerm SurfaceTerm
    | SLam String SurfaceTerm SurfaceTerm
    | SApp SurfaceTerm SurfaceTerm
    deriving (Show, Eq)
