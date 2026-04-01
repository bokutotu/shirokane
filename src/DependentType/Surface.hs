module DependentType.Surface where

data SurfaceTerm
    = SType
    | SVar String
    | SLam String SurfaceTerm SurfaceTerm
    | SApp SurfaceTerm SurfaceTerm
    | SAnn SurfaceTerm SurfaceTerm
    | SPi String SurfaceTerm SurfaceTerm
    | SSigma String SurfaceTerm SurfaceTerm
    | SPair SurfaceTerm SurfaceTerm
    | SFst SurfaceTerm
    | SSnd SurfaceTerm
    deriving (Show, Eq)
